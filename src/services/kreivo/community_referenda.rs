use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use sube::{sube, Response};

use crate::services::kreivo::community_track::ChainStateError;

#[derive(Debug, Deserialize)]
pub struct TrackInfo {
    pub id: u32,
    pub count: u32,
}

pub async fn track_queue(item: u16) -> Result<Vec<TrackInfo>, ChainStateError> {
    let query = format!(
        "wss://kreivo.io/communityReferenda/trackQueue/{}",
        item
    );
    let response = sube!(&query)
        .await
        .map_err(|_| ChainStateError::FailedQuery)?;

    let Response::Value(ref value) = response else {
        return Err(ChainStateError::InternalError);
    };

    let Ok(value) = serde_json::to_value(&value) else {
        return Err(ChainStateError::InternalError);
    };

    let Value::Array(track_infos) = value else {
        return Err(ChainStateError::InternalError);
    };

    let track_infos: Vec<TrackInfo> = track_infos
        .into_iter()
        .map(|item| from_value(item).expect("invalid format for track info"))
        .collect();

    Ok(track_infos)
}

pub async fn referendum_count() -> Result<u16, ChainStateError> {
    let query = format!("wss://kreivo.io/communityReferenda/referendumCount");
    let response = sube!(&query)
        .await
        .map_err(|_| ChainStateError::FailedQuery)?;

    let Response::Value(ref value) = response else {
        return Err(ChainStateError::InternalError);
    };

    let Ok(value) = serde_json::to_value(&value) else {
        return Err(ChainStateError::InternalError);
    };

    let count = serde_json::from_value::<u16>(value).map_err(|_| ChainStateError::FailedDecode)?;

    Ok(count)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Origin {
    #[serde(rename = "Communities")]
    pub communities: Communities,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Communities {
    #[serde(rename = "community_id")]
    pub community_id: u16,
    pub subset: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Deposit {
    pub who: Vec<u8>,
    pub amount: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Deciding {
    pub since: u32,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Tally {
    pub ayes: u64,
    pub nays: u64,
    #[serde(rename = "bare_ayes")]
    pub bare_ayes: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Alarm {
    Single(u32),
    Multiple(Vec<u32>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ongoing {
    pub track: u32,
    pub origin: Origin,
    pub submitted: u32,
    pub submission_deposit: Deposit,
    pub deciding: Option<Deciding>,
    pub in_queue: bool,
    pub tally: Tally,
    pub alarm: Option<Vec<Alarm>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OngoingWrapper {
    #[serde(rename = "Ongoing")]
    pub ongoing: Ongoing,
}

pub async fn referendum_info_for(id: u16) -> Result<OngoingWrapper, ChainStateError> {
    let query = format!(
        "wss://kreivo.io/communityReferenda/referendumInfoFor/{}",
        id
    );
    let response = sube!(&query)
        .await
        .map_err(|_| ChainStateError::FailedQuery)?;

    let Response::Value(ref value) = response else {
        return Err(ChainStateError::InternalError);
    };

    let Ok(value) = serde_json::to_value(&value) else {
        return Err(ChainStateError::InternalError);
    };

    let initiative = serde_json::from_value::<OngoingWrapper>(value)
        .map_err(|_| ChainStateError::FailedDecode)?;

    Ok(initiative)
}

pub async fn metadata_of(id: u16) -> Result<Vec<u8>, ChainStateError> {
    let query = format!("wss://kreivo.io/communityReferenda/metadataOf/{}", id);
    let response = sube!(&query)
        .await
        .map_err(|_| ChainStateError::FailedQuery)?;

    let Response::Value(ref value) = response else {
        return Err(ChainStateError::InternalError);
    };

    let Ok(value) = serde_json::to_value(&value) else {
        return Err(ChainStateError::InternalError);
    };

    let preimage_hash =
        serde_json::from_value::<Vec<u8>>(value).map_err(|_| ChainStateError::FailedDecode)?;

    Ok(preimage_hash)
}

pub async fn get_initiatives_by_community(
    track_id: u16,
) -> Result<Vec<TrackInfo>, ChainStateError> {
    let track_infos = track_queue(track_id).await?;

    Ok(track_infos)
}
