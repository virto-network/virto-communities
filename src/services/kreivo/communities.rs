use serde::Deserialize;
use sube::{sube, Response};
#[derive(Debug, Deserialize)]
pub struct CommunityTracks {
    pub communities: Vec<u16>,
}
#[derive(Debug)]
pub enum ChainStateError {
    FailedQuery,
    InternalError,
    FailedDecode,
}
pub async fn is_admin(address: &[u8]) -> Result<bool, ChainStateError> {
    let response = sube!("wss://kreivo.io/communities/communityIdFor").await.map_err(|_| ChainStateError::FailedQuery)?;
    let Response::ValueSet(value) = response else {
        return Err(ChainStateError::InternalError);
    };
    for d in value.iter() {
        let Some(value) = d.0.first() else {
            continue;
        };
        let Ok(value) = serde_json::to_value(value) else {
            continue;
        };
        let Some(system) = value.get("system") else {
            continue;
        };
        let Some(signed) = system.get("Signed") else {
            continue;
        };
        let Ok(value) = serde_json::to_value(signed) else {
            continue;
        };
        let Ok(account_info) = serde_json::from_value::<Vec<u8>>(value) else {
            continue;
        };
        if address == account_info {
            return Ok(true);
        }
    }
    Ok(false)
}
