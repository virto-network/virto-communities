use sube::{sube, Response};
use crate::services::kreivo::community_track::ChainStateError;
pub async fn preimage_for(hash: &str, len: u32) -> Result<String, ChainStateError> {
    let query = format!("wss://kreivo.io/preimage/preimageFor/{}/{}", hash, len);
    let response = sube!(& query).await.map_err(|_| ChainStateError::FailedQuery)?;
    let Response::Value(ref value) = response else {
        return Err(ChainStateError::InternalError);
    };
    let Ok(value) = serde_json::to_value(&value) else {
        return Err(ChainStateError::InternalError);
    };
    let response = serde_json::from_value::<Vec<u8>>(value)
        .map_err(|_| ChainStateError::FailedDecode)?;
    let room_id = String::from_utf8(response)
        .map_err(|_| ChainStateError::FailedDecode)?;
    Ok(room_id)
}
pub async fn request_status_for(hash: &str) -> Result<u32, ChainStateError> {
    let query = format!("wss://kreivo.io/preimage/requestStatusFor/{}", hash);
    let response = sube!(& query).await.map_err(|_| ChainStateError::FailedQuery)?;
    let Response::Value(ref value) = response else {
        return Err(ChainStateError::InternalError);
    };
    let Ok(value) = serde_json::to_value(&value) else {
        return Err(ChainStateError::InternalError);
    };
    let response = value.get("Unrequested").ok_or(ChainStateError::FailedDecode)?;
    let len = response.get("len").ok_or(ChainStateError::FailedDecode)?;
    let len = serde_json::from_value::<u32>(len.clone())
        .map_err(|_| ChainStateError::FailedDecode)?;
    Ok(len)
}
