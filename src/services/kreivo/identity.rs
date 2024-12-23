use serde::Deserialize;
use sube::{sube, Response};
use super::community_track::ChainStateError;
#[derive(Debug, Deserialize)]
pub struct Invitation {
    pub address: Vec<u8>,
}
pub async fn superOf(account: &str) -> Result<Invitation, ChainStateError> {
    let query = format!("wss://people-kusama-rpc.dwellir.com/identity/superOf/{}", account);

    let response = sube!(&query).await.map_err(|e| {
        dioxus::logger::tracing::info!("{:?}", e);
        ChainStateError::FailedQuery
    })?;

    let Response::Value(value) = response else {
        return Err(ChainStateError::InternalError);
    };
    let value = serde_json::to_value(&value).expect("it must be a serialized object");
    dioxus::logger::tracing::info!("super of: {:?}", value);
    let account_info = serde_json::from_value::<Invitation>(value)
        .map_err(|_| ChainStateError::FailedDecode)?;
    Ok(account_info)
}
pub async fn identityOf(account: &str) -> Result<String, ChainStateError> {
    let query = format!(
        "wss://people-kusama-rpc.dwellir.com/identity/identityOf/{}",
        account,
    );
    let response = sube!(& query).await.map_err(|_| ChainStateError::FailedQuery)?;
    let Response::Value(value) = response else {
        return Err(ChainStateError::InternalError);
    };
    let value = serde_json::to_value(&value).expect("it must be a serialized object");
    let account_info = value
        .get(0)
        .ok_or(ChainStateError::FailedDecode)?
        .get("info")
        .ok_or(ChainStateError::FailedDecode)?
        .get("web")
        .ok_or(ChainStateError::FailedDecode)?;
    dioxus::logger::tracing::info!("{:?}", account_info);
    let raw_value = value.get("Raw30").expect("Expected Raw19 key");
    if let Some(array) = raw_value.as_array() {
        let result: String = array
            .iter()
            .filter_map(|v| v.as_u64())
            .map(|v| v as u8 as char)
            .collect();
        dioxus::logger::tracing::info!("{}", result);
    }
    let account_info = serde_json::from_value(account_info.clone())
        .map_err(|_| ChainStateError::FailedDecode)?;
    Ok(account_info)
}
