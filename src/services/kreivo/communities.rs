use codec::Decode;
use serde::Deserialize;
use sube::{sube, Response};

#[derive(Decode, Debug, Deserialize)]
pub struct CommunityTracks {
    pub communities: Vec<u16>,
}

#[derive(Debug)]
pub enum ChainStateError {
    FailedQuery,
    InternalError,
    FailedDecode,
}

pub async fn communityIdForSigned() -> Result<Vec<Vec<u8>>, ChainStateError> {
    let query = format!("wss://kreivo.io/communities/communityIdFor");

    let response = sube!(&query)
        .await
        .map_err(|_| ChainStateError::FailedQuery)?;

    let Response::ValueSet(value) = response else {
        return Err(ChainStateError::InternalError);
    };

    let mut values = vec![];
    for d in value.iter() {
        let Some(value) = d.0.get(0) else {
            continue;
        };

        let Ok(value) = serde_json::to_value(&value) else {
            continue;
        };

        let Some(system) = value.get("system") else {
            continue;
        };

        let Some(signed) = system.get("Signed") else {
            continue;
        };

        let Ok(value) = serde_json::to_value(&signed) else {
            continue;
        };
        let Ok(account_info) = serde_json::from_value::<Vec<u8>>(value) else {
            continue;
        };
        values.push(account_info)
    }

    Ok(values)
}
