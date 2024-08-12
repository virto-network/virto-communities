use sube::{sube, Response};

use crate::services::kreivo::communities::ChainStateError;

pub async fn number() -> Result<u32, ChainStateError> {
    let query = format!("wss://kusama-rpc.dwellir.com/system/number");

    let response = sube!(&query)
        .await
        .map_err(|_| ChainStateError::FailedQuery)?;

    let Response::Value(value) = response else {
        return Err(ChainStateError::InternalError);
    };

    let value = serde_json::to_value(&value).map_err(|_| ChainStateError::FailedDecode)?;
    let number =
        serde_json::from_value::<u32>(value).map_err(|_| ChainStateError::FailedDecode)?;

    Ok(number)
}
