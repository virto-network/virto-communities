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

pub async fn tracksIds() -> Result<CommunityTracks, ChainStateError> {
    let query = format!("wss://kreivo.io/communityTracks/tracksIds");

    let response = sube!(&query)
        .await
        .map_err(|_| ChainStateError::FailedQuery)?;

    let Response::Value(value) = response else {
        return Err(ChainStateError::InternalError);
    };

    let data = value.as_ref();
    let account_info =
        CommunityTracks::decode(&mut &data[..]).map_err(|_| ChainStateError::FailedDecode)?;

    Ok(account_info)
}

const DEFAULT_MAX_TRACK_NAME_LEN: usize = 25;
const N: usize = DEFAULT_MAX_TRACK_NAME_LEN;

#[derive(Decode, Debug, Deserialize)]
pub struct TrackInfo {
    pub name: [u8; N],
}

pub async fn tracks(track: u16) -> Result<TrackInfo, ChainStateError> {
    let query = format!("wss://kreivo.io/communityTracks/tracks/{}", track);

    let response = sube!(&query).await.map_err(|e| {
        log::info!("{}", e);
        ChainStateError::FailedQuery
    })?;

    let Response::Value(value) = response else {
        return Err(ChainStateError::InternalError);
    };

    let data = value.as_ref();
    let account_info =
        TrackInfo::decode(&mut &data[..]).map_err(|_| ChainStateError::FailedDecode)?;

    Ok(account_info)
}
