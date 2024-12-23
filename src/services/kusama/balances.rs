use sube::{sube, Response};

use crate::services::kreivo::{balances::AccountInfo, community_track::ChainStateError};

pub async fn account(account: &str) -> Result<AccountInfo, ChainStateError> {
    let query = format!("wss://kusama-rpc.dwellir.com/system/account/{}", account);

    dioxus::logger::tracing::info!("query: {:#?}", query);
    let response = sube!(&query).await.map_err(|_| {
        ChainStateError::FailedQuery
    })?;

    let Response::Value(value) = response else {
        return Err(ChainStateError::InternalError);
    };

    let value = serde_json::to_value(&value).map_err(|_| ChainStateError::FailedDecode)?;
    let account_info =
        serde_json::from_value::<AccountInfo>(value).map_err(|_| ChainStateError::FailedDecode)?;

    Ok(account_info)
}
