use codec::Decode;
use serde::Deserialize;
use sube::{sube, Response};

use crate::{pages::dashboard::Community, services::kreivo::community_track::ChainStateError};

use super::community_track::{tracks, tracksIds};

#[derive(Decode, Debug, Deserialize)]
pub struct CollectionDetails {
    pub items: u16,
    pub item_metadatas: u16,
    pub attributes: u16,
    pub owner: Vec<u8>,
}

pub async fn collection(collection: u16) -> Result<CollectionDetails, ChainStateError> {
    let query = format!(
        "wss://kreivo.io/communityMemberships/collection/{}",
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
    let query = format!("wss://kreivo.io/communityMemberships/item/{}", item);
    let response = sube!(&query)
        .await
        .map_err(|_| ChainStateError::FailedQuery)?;

    let Response::ValueSet(value) = response else {
        return Err(ChainStateError::InternalError);
    };

    Ok(value.len() as u16)
}

pub async fn get_membership_id(address: &str, community_id: u16) -> Result<u16, ChainStateError> {
    let query = format!(
        "wss://kreivo.io/communityMemberships/account/{}/{}",
        address, community_id
    );
    let response = sube!(&query)
        .await
        .map_err(|_| ChainStateError::FailedQuery)?;

    let Response::ValueSet(ref value) = response else {
        return Err(ChainStateError::InternalError);
    };

    // Get the first membership
    let Some(value) = value.get(0) else {
        return Err(ChainStateError::InternalError);
    };

    let value = &value.0;

    // Get the membership id
    let Some(value) = value.get(2) else {
        return Err(ChainStateError::InternalError);
    };

    let Ok(value) = serde_json::to_value(&value) else {
        return Err(ChainStateError::InternalError);
    };

    let membership_id =
        serde_json::from_value::<u16>(value).map_err(|_| ChainStateError::FailedDecode)?;

    Ok(membership_id)
}

pub async fn get_owned_memberships(address: &str) -> Result<u16, ChainStateError> {
    let query = format!("wss://kreivo.io/communityMemberships/account/{}", address);
    let response = sube!(&query)
        .await
        .map_err(|_| ChainStateError::FailedQuery)?;

    let Response::ValueSet(ref value) = response else {
        return Err(ChainStateError::InternalError);
    };

    // // Get the first membership
    // let Some(value) = value.get(0) else {
    //     return Err(ChainStateError::InternalError);
    // };

    // let value = &value.0;

    // // Get the membership id
    // let Some(value) = value.get(2) else {
    //     return Err(ChainStateError::InternalError);
    // };

    // let Ok(value) = serde_json::to_value(&value) else {
    //     return Err(ChainStateError::InternalError);
    // };

    // let membership_id =
    //     serde_json::from_value::<u16>(value).map_err(|_| ChainStateError::FailedDecode)?;

    Ok(value.len() as u16)
}

pub async fn get_communities_by_member(member: &[u8]) -> Result<Vec<Community>, ChainStateError> {
    let mut communities = vec![];

    let address = format!("0x{}", hex::encode(member));
    log::info!("address: {address}, member {member:?}");
    let community_trackIds = tracksIds().await.map_err(|e| {
        log::warn!("error: {:?}", e);
        ChainStateError::FailedQuery
    })?;

    for community in community_trackIds.communities.iter() {
        log::info!("address: {address}, community {community}");
        let query = format!(
            "wss://kreivo.io/communityMemberships/account/{}/{}",
            address, community
        );

        let response = sube!(&query).await.map_err(|e| {
            log::warn!("error: {:?}", e);
            ChainStateError::FailedQuery
        })?;

        log::info!("{:?}", response);
        let Response::ValueSet(value) = response else {
            continue;
        };

        if value.len() > 0 {
            let response_track = tracks(*community).await;
            log::info!("{:?}", response_track);
            let response_collection = collection(*community).await;
            log::info!("{:?}", response_collection);
            let response_item = item(*community, None).await;
            log::info!("{:?}", response_item);

            let collection_items = match response_collection {
                Ok(ref collection) => {
                    let address = format!("0x{}", hex::encode(collection.owner.clone()));
                    get_owned_memberships(&address).await.unwrap_or(0u16)
                }
                Err(_) => 0u16,
            };

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

            let item_details = match response_item {
                Ok(items) => items,
                Err(_) => 0u16,
            };

            let community = Community {
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
