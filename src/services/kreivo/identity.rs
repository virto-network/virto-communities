use serde::Deserialize;
use sube::{sube, Response};

use super::community_track::ChainStateError;

#[derive(Debug, Deserialize)]
pub struct Invitation {
    pub address: Vec<u8>,
    pub raw: Raw,
}

#[derive(Debug, Deserialize)]
struct Raw {
    #[serde(rename = "Raw6")]
    pub raw6: Vec<u8>,
}

pub async fn superOf(account: &str) -> Result<Invitation, ChainStateError> {
    let query = format!("wss://people-kusama-rpc.dwellir.com/identity/superOf/0x6d6f646c6b762f636d7479730200000000000000000000000000000000000000");

    // log::info!("query: {:#?}", query);
    let response = sube!(&query).await.map_err(|e| {
        log::info!("{:?}", e);
        ChainStateError::FailedQuery
    })?;

    let Response::Value(value) = response else {
        return Err(ChainStateError::InternalError);
    };

    let value = serde_json::to_value(&value).expect("it must be a serialized object");
    log::info!("super of: {:?}", value);
    let account_info =
        serde_json::from_value::<Invitation>(value).map_err(|_| ChainStateError::FailedDecode)?;

    Ok(account_info)
}

pub async fn identityOf(account: &str) -> Result<String, ChainStateError> {
    let query = format!("ws://127.0.0.1:11004/identity/identityOf/{}", account);

    log::info!("query: {:#?}", query);
    let response = sube!(&query)
        .await
        .map_err(|_| ChainStateError::FailedQuery)?;

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

    log::info!("{:?}", account_info);

    // Convertir el valor "web" a un string JSON
    let raw_value = value.get("Raw30").expect("Expected Raw19 key");

    if let Some(array) = raw_value.as_array() {
        // Convertir el array de números en un string
        let result: String = array
            .iter()
            .filter_map(|v| v.as_u64())
            .map(|v| v as u8 as char)
            .collect();

        log::info!("{}", result);
    }

    let account_info =
        serde_json::from_value(account_info.clone()).map_err(|_| ChainStateError::FailedDecode)?;

    Ok(account_info)
}
