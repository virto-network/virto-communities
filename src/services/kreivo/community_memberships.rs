use codec::Decode;
use serde::Deserialize;
use sube::{sube, Response};

use crate::{pages::dashboard::Community, services::kreivo::community_track::ChainStateError};

use super::community_track::{tracks, tracksIds, TrackInfo};

#[derive(Decode, Debug, Deserialize)]
pub struct CollectionDetails {
    pub items: u16,
    pub item_metadatas: u16,
    pub owner: Vec<u8>,
}

pub async fn collection(collection: u16) -> Result<CollectionDetails, ChainStateError> {
    let query = format!(
        "ws://localhost:12281/communityMemberships/collection/{}",
        collection
    );

    let response = sube!(&query)
        .await
        .map_err(|_| ChainStateError::FailedQuery)?;

    let Response::Value(value) = response else {
        return Err(ChainStateError::InternalError);
    };

    let value = serde_json::to_value(&value).map_err(|_| ChainStateError::FailedDecode)?;
    let account_info = serde_json::from_value::<CollectionDetails>(value)
        .map_err(|_| ChainStateError::FailedDecode)?;

    Ok(account_info)
}

pub async fn item(item: u16, member: Option<u16>) -> Result<u16, ChainStateError> {
    let query = format!("ws://localhost:12281/communityMemberships/item/{}", item);
    let response = sube!(&query)
        .await
        .map_err(|_| ChainStateError::FailedQuery)?;

    let Response::ValueSet(value) = response else {
        return Err(ChainStateError::InternalError);
    };

    Ok(value.len() as u16)
}

pub async fn get_communities_by_member(member: &[u8]) -> Result<Vec<Community>, ChainStateError> {
    let mut communities = vec![];

    let address = format!("0x{}", hex::encode(member));
    let community_trackIds = tracksIds()
        .await
        .map_err(|_| ChainStateError::FailedQuery)?;

    for community in community_trackIds.communities.iter() {
        let query = format!(
            "ws://127.0.0.1:12281/communityMemberships/account/{}/{}",
            address, community
        );

        let response = sube!(&query)
            .await
            .map_err(|_| ChainStateError::FailedQuery)?;

        let Response::ValueSet(value) = response else {
            return Err(ChainStateError::InternalError);
        };

        if value.len() > 0 {
            let response_track = tracks(*community).await;
            let response_collection = collection(*community).await;
            let response_item = item(*community, None).await;

            let Ok(track_info) = response_track else {
                continue;
            };

            let filtered_name = track_info
                .name
                .iter()
                .filter(|b| **b != 0)
                .cloned()
                .collect::<Vec<_>>();

            let filtered_name: &[u8] = &filtered_name;

            let collection_items = match response_collection {
                Ok(ref details) => details.items.clone(),
                Err(_) => 0u16,
            };

            let item_details = match response_item {
                Ok(items) => items,
                Err(_) => 0u16,
            };

            let mut community = Community {
                id: *community,
                icon: None,
                name: String::from_utf8_lossy(filtered_name).to_string(),
                description: String::from(""),
                tags: vec![],
                memberships: collection_items,
                members: item_details,
            };

            communities.push(community)
        }
    }

    Ok(communities)
}
