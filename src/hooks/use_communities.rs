use std::{str::FromStr, vec};

use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use gloo::storage::{errors::StorageError, LocalStorage};

use crate::{
    pages::dashboard::Community,
    services::kreivo::community_memberships::{get_communities, is_community_member_by_address},
};

use super::{
    use_notification::use_notification,
    use_session::use_session,
    use_tooltip::{use_tooltip, TooltipItem},
};

pub type Communities = Vec<Community>;
pub fn use_communities() -> UseCommunitiesState {
    let i18 = use_i18();
    let session = use_session();
    let mut tooltip = use_tooltip();
    let mut notification = use_notification();

    let mut communities = consume_context::<Signal<Communities>>();
    let community = consume_context::<Signal<Community>>();

    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        tooltip.handle_tooltip(TooltipItem {
            title: translate!(i18, "dao.tips.loading.title"),
            body: translate!(i18, "dao.tips.loading.description"),
            show: true,
        });

        let cached_communitites = get_cached_communities();

        communities.set(cached_communitites);

        let Some(session) = session.get() else {
            log::info!("error here by account");
            notification.handle_error(&translate!(i18, "errors.communities.query_failed"));
            tooltip.hide();
            return;
        };

        let Ok(public_address) = sp_core::sr25519::Public::from_str(&session.address) else {
            log::info!("error here by address");
            notification.handle_error(&translate!(i18, "errors.wallet.account_address"));
            tooltip.hide();
            return;
        };

        let Ok(mut community_tracks) = get_communities().await else {
            log::info!("error here by memeber");
            notification.handle_error(&translate!(i18, "errors.communities.query_failed"));
            tooltip.hide();
            return;
        };

        let mut temporal_favorite_communities = get_favorite_communities();

        for community in &mut community_tracks {
            let Ok(is_member) =
                is_community_member_by_address(&public_address.0, community.id).await
            else {
                continue;
            };

            community.has_membership = is_member;

            if !is_member {
                let mut to_remove = None;

                for (index, favorite) in temporal_favorite_communities.iter().enumerate() {
                    let is_community_favorite = community.id == *favorite;
                    community.favorite = is_community_favorite;

                    if is_community_favorite {
                        to_remove = Some(index);
                        break;
                    }
                }

                if let Some(index) = to_remove {
                    temporal_favorite_communities.remove(index);
                }
            }
        }

        communities.set(community_tracks.clone());

        let Ok(cached_communitites) = serde_json::to_string(&community_tracks) else {
            return;
        };

        if let Err(_) =
            <LocalStorage as gloo::storage::Storage>::set("communities", cached_communitites)
        {
            log::warn!("Failed to persist communities");
        };

        tooltip.hide();
    });

    use_hook(|| UseCommunitiesState {
        communities,
        community,
    })
}
#[derive(Clone, Copy)]
pub struct UseCommunitiesState {
    communities: Signal<Communities>,
    community: Signal<Community>,
}

pub enum CommunitiesError {
    NotFound,
    FailedUpdatingFavorites,
    NotFoundFavorite,
}

impl UseCommunitiesState {
    pub fn get_communities(&self) -> Vec<Community> {
        self.communities.read().clone()
    }

    pub fn get_communities_by_filters(
        &self,
        filter_by_member: Option<()>,
        filter_by_name: Option<&str>,
        filter_by_pagination: Option<(usize, usize)>,
    ) -> Vec<Community> {
        let communities = self.communities.read().clone();

        let communities = communities
            .into_iter()
            .filter(|community| {
                if filter_by_member.is_some() {
                    return community.has_membership || community.favorite;
                }

                true
            })
            .collect::<Vec<Community>>();

        match filter_by_name {
            Some(name) => communities
                .into_iter()
                .filter(|community| community.name.to_lowercase().contains(&name))
                .collect::<Vec<Community>>(),
            None => {
                if let Some((from, to)) = filter_by_pagination {
                    if to > communities.len() {
                        communities[from..communities.len()].to_vec()
                    } else {
                        communities[from..to].to_vec()
                    }
                } else {
                    communities
                }
            }
        }
    }

    pub fn set_community(&mut self, id: u16) -> Result<(), CommunitiesError> {
        let mut c = self.community.write();

        let position = self
            .communities
            .read()
            .iter()
            .position(|community| community.id == id)
            .ok_or(CommunitiesError::NotFound)?;

        *c = self.communities.read()[position].clone();
        Ok(())
    }

    pub fn handle_favorite(&mut self, id: u16) -> Result<(), CommunitiesError> {
        let Some(position) = self
            .communities
            .read()
            .iter()
            .position(|community| community.id == id)
        else {
            return Err(CommunitiesError::NotFoundFavorite);
        };

        let is_favorite = !self.communities.read()[position].favorite;

        let mut temporal_favorite_communities = get_favorite_communities();

        if is_favorite {
            temporal_favorite_communities.push(id);
        } else {
            let Some(position) = temporal_favorite_communities
                .iter()
                .position(|identifier| *identifier == id)
            else {
                return Err(CommunitiesError::NotFoundFavorite);
            };

            temporal_favorite_communities.remove(position);
        }

        let Ok(temporal_favorite_communities) =
            serde_json::to_string(&temporal_favorite_communities)
        else {
            return Err(CommunitiesError::FailedUpdatingFavorites);
        };

        <LocalStorage as gloo::storage::Storage>::set(
            "favorite_communities",
            temporal_favorite_communities,
        )
        .map_err(|_| CommunitiesError::FailedUpdatingFavorites)?;

        self.communities.write()[position].favorite = is_favorite;

        let Ok(cached_communitites) = serde_json::to_string(&*self.communities.read()) else {
            return Err(CommunitiesError::FailedUpdatingFavorites);
        };

        if let Err(_) =
            <LocalStorage as gloo::storage::Storage>::set("communities", cached_communitites)
        {
            log::warn!("Failed to persist communities");
        };


        Ok(())
    }

    pub fn remove_community(&mut self) {
        let mut c = self.community.write();
        *c = Community::default()
    }
    pub fn get_community(&self) -> Community {
        self.community.read().clone()
    }
}

fn get_favorite_communities() -> Vec<u16> {
    let favorite_communities: Result<String, StorageError> =
        <LocalStorage as gloo::storage::Storage>::get("favorite_communities");

    let Ok(favorite_communities) = favorite_communities else {
        return vec![];
    };

    let Ok(favorite_communities) = serde_json::from_str::<Vec<u16>>(&favorite_communities) else {
        return vec![];
    };

    favorite_communities
}

fn get_cached_communities() -> Vec<Community> {
    let communities: Result<String, StorageError> =
        <LocalStorage as gloo::storage::Storage>::get("communities");

    let Ok(communities) = communities else {
        return vec![];
    };

    let Ok(communities) = serde_json::from_str::<Vec<Community>>(&communities) else {
        return vec![];
    };

    communities
}
