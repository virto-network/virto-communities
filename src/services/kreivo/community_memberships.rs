use codec::Decode;
use serde::Deserialize;
use sube::{sube, Response};

use crate::services::kreivo::community_track::ChainStateError;

#[derive(Decode, Debug, Deserialize)]
pub struct CollectionDetails {
    pub items: u16,
    pub item_metadatas: u16,
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
