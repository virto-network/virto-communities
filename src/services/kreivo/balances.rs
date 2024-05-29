use serde::Deserialize;
use sube::{sube, Response};

use super::community_track::ChainStateError;

#[derive(Debug, Deserialize)]
pub struct AccountInfo {
    pub nonce: u64,
    pub consumers: u64,
    pub providers: u64,
    pub sufficients: u64,
    pub data: AccountData,
}

#[derive(Debug, Deserialize)]
pub struct AccountData {
    pub free: u128,
    pub reserved: u128,
    pub frozen: u128,
    pub flags: u128,
}

pub async fn account(account: &str) -> Result<AccountInfo, ChainStateError> {
    let query = format!("wss://kreivo.kippu.rocks/system/account/{}", account);

    log::info!("query: {:#?}", query);
    let response = sube!(&query).await.map_err(|_| {
        ChainStateError::FailedQuery
    })?;

    let Response::Value(value) = response else {
        return Err(ChainStateError::InternalError);
    };

    let value = serde_json::to_value(&value).expect("it must be a serialized object");
    let account_info =
        serde_json::from_value::<AccountInfo>(value).map_err(|_| ChainStateError::FailedDecode)?;

    Ok(account_info)
}
