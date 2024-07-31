use std::str::FromStr;

use dioxus::prelude::*;
use sp_core::sr25519::Public;

use crate::{
    hooks::{
        use_accounts::{use_accounts, UseAccountsState},
        use_notification::{use_notification, UseNotificationState},
        use_tooltip::{UseTooltipState, TooltipItem},
    },
    services::kreivo::community_memberships::get_communities_by_member,
    services::kreivo::{
        community_memberships::{collection, item},
        community_track::{tracks, tracksIds},
        identity::{identityOf, superOf},
    }
};

#[derive(Debug, Clone)]
pub enum CommunityServiceError {
    AccountError,
    AddressError,
    QueryFailed,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Community {
    pub id: u16,
    pub icon: Option<String>,
    pub name: String,
    pub description: String,
    pub memberships: u16,
    pub tags: Vec<String>,
    pub members: u16,
}

pub async fn fetch_communities(
    accounts: &UseAccountsState,
    notification: &UseNotificationState,
    tooltip: &mut UseTooltipState,
    is_loading: &mut dioxus::prelude::Signal<bool>,
) -> Result<Vec<Community>, CommunityServiceError> {
    is_loading.set(true);
    tooltip.handle_tooltip(TooltipItem {
        title: "Fetching Communities".to_string(),
        body: "Please wait while we fetch the communities.".to_string(),
        show: true,
    });

    let account = accounts.get_account().ok_or(CommunityServiceError::AccountError)?;
    let address = Public::from_str(&account.address()).map_err(|_| CommunityServiceError::AddressError)?;

    let community_tracks = get_communities_by_member(&address.0)
        .await
        .map_err(|_| CommunityServiceError::QueryFailed)?;

    tooltip.hide();
    is_loading.set(false);
    Ok(community_tracks)
}

pub async fn fetch_community_ids(
    notification: &UseNotificationState,
    tooltip: &mut UseTooltipState,
    is_loading: &mut dioxus::prelude::Signal<bool>,
) -> Result<Vec<u16>, CommunityServiceError> {
    is_loading.set(true);
    tooltip.handle_tooltip(TooltipItem {
        title: "Fetching Community IDs".to_string(),
        body: "Please wait while we fetch the community IDs.".to_string(),
        show: true,
    });

    let community_tracks = tracksIds()
        .await
        .map_err(|_| CommunityServiceError::QueryFailed)?;
    
    is_loading.set(false);
    tooltip.hide();
    Ok(community_tracks.communities)
}