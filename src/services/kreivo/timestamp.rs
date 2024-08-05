use serde::Deserialize;
use sube::{sube, Response};

use super::community_track::ChainStateError;

pub async fn now() -> Result<u64, ChainStateError> {
    let query = format!("ws://localhost:12281/timestamp/now");

    let response = sube!(&query)
        .await
        .map_err(|_| ChainStateError::FailedQuery)?;

    let Response::Value(value) = response else {
        return Err(ChainStateError::InternalError);
    };

    let value = serde_json::to_value(&value).map_err(|_| ChainStateError::FailedDecode)?;
    let timestamp =
        serde_json::from_value::<u64>(value).map_err(|_| ChainStateError::FailedDecode)?;

    Ok(timestamp)
}
