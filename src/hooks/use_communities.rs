use sp_core::crypto::Ss58Codec;
use std::vec;

use dioxus::{logger::tracing::{debug, warn}, prelude::*};
use dioxus_i18n::t;
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
    let session = use_session();
    let mut tooltip = use_tooltip();
    let mut notification = use_notification();

    let mut communities = consume_context::<Signal<Communities>>();
    let community = consume_context::<Signal<Community>>();
    let mut is_loading = use_signal(|| false);

    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        tooltip.handle_tooltip(TooltipItem {
            title: t!("dashboard-tips-loading-title"),
            body: t!("dashboard-tips-loading-description"),
            show: true,
        });

        let cached_communities = get_cached_communities();
        communities.set(cached_communities.clone());

        if cached_communities.is_empty() {
            is_loading.set(true);
        } else {
            tooltip.hide();
        }

        let public_address = session.get().and_then(|session| {
            match sp_core::sr25519::Public::from_ss58check(&session.address) {
                Ok(public_address) => Some(public_address.0),
                Err(_) => {
                    warn!("error here by address");
                    notification.handle_error(&t!("errors-wallet-account_address"));
                    None
                }
            }
        });

        let Ok(mut community_tracks) = get_communities().await else {
            warn!("error here by member");
            notification.handle_error(&t!("errors-communities-query_failed"));
            tooltip.hide();
            is_loading.set(false);
            return;
        };

        let mut temporal_favorite_communities = get_favorite_communities();

        for community in &mut community_tracks {
            let is_member = match public_address {
                Some(public_address) => {
                    is_community_member_by_address(&public_address, community.id)
                        .await
                        .unwrap_or(false)
                }
                None => false,
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

        if let Ok(cached_communities) = serde_json::to_string(&community_tracks) {
            if let Err(e) =
                <LocalStorage as gloo::storage::Storage>::set("communities", cached_communities)
            {
                warn!("Failed to persist communities: {:?}", e);
            }
        }

        tooltip.hide();
        is_loading.set(false);
    });

    use_hook(|| UseCommunitiesState {
        communities,
        community,
        is_loading,
    })
}
#[derive(Clone, Copy)]
pub struct UseCommunitiesState {
    communities: Signal<Communities>,
    community: Signal<Community>,
    pub is_loading: Signal<bool>,
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
        if *self.is_loading.read() {
            return vec![];
        }

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
                .filter(|community| community.name.to_lowercase().contains(name))
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

        let Ok(cached_communities) = serde_json::to_string(&*self.communities.read()) else {
            return Err(CommunitiesError::FailedUpdatingFavorites);
        };

        if <LocalStorage as gloo::storage::Storage>::set("communities", cached_communities).is_err()
        {
            warn!("Failed to persist communities");
        };

        Ok(())
    }

    pub fn remove_community(&mut self) {
        *self.community.write() = Community::default();
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

pub fn get_cached_communities() -> Vec<Community> {
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
