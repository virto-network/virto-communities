use dioxus::logger::tracing::{debug, warn};
use serde::Deserialize;
use sube::{sube, Response};

use super::communities::CommunityTracks;
#[derive(Debug)]
pub enum ChainStateError {
    FailedQuery,
    InternalError,
    FailedDecode,
}
pub async fn tracksIds() -> Result<CommunityTracks, ChainStateError> {
    let response = sube!("wss://kreivo.io/communityTracks/tracksIds")
        .await
        .map_err(|e| {
            warn!("{:?}", e);
            ChainStateError::FailedQuery
        })?;
    let Response::Value(value) = response else {
        return Err(ChainStateError::InternalError);
    };
    let ids = serde_json::from_value::<Vec<u16>>(value.into())
        .map_err(|e| {
            warn!("{:?}", e);
            ChainStateError::FailedDecode
        })?;
    let community_tracks = CommunityTracks{communities: ids};
    Ok(community_tracks)
}
const DEFAULT_MAX_TRACK_NAME_LEN: usize = 25;
const N: usize = DEFAULT_MAX_TRACK_NAME_LEN;
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Curve {
    LinearDecreasing { ceil: u64, floor: u64, length: u64 },
    SteppedDecreasing { begin: u64, end: u64, step: u64, period: u64 },
    Reciprocal { factor: i64, x_offset: i64, y_offset: i64 },
}
impl Curve {
    pub fn calculate_threshold(&self, progress: f64) -> f64 {
        match self {
            Curve::LinearDecreasing { ceil, floor, length } => {
                let length = *length as f64 / 10_000_000.0;
                let ceil = *ceil as f64 / 10_000_000.0;
                let floor = *floor as f64 / 10_000_000.0;
                let progress = progress / (length / 100.0);
                ceil - progress * (ceil - floor)
            }
            _ => 100.0,
        }
    }
}
#[derive(Debug, Deserialize)]
pub struct TrackInfo {
    pub name: [u8; N],
    #[serde(rename = "decision_period")]
    pub decision_period: u32,
    #[serde(rename = "min_approval")]
    pub min_approval: Curve,
    #[serde(rename = "min_support")]
    pub min_support: Curve,
}
pub async fn tracks(track: u16) -> Result<TrackInfo, ChainStateError> {
    let query = format!("wss://kreivo.io/communityTracks/tracks/{}", track);
    let response = sube!(& query)
        .await
        .map_err(|e| {
            debug!("{}", e);
            ChainStateError::FailedQuery
        })?;
    let Response::Value(value) = response else {
        return Err(ChainStateError::InternalError);
    };
    let Ok(value) = serde_json::to_value(&value) else {
        return Err(ChainStateError::InternalError);
    };
    let account_info = serde_json::from_value::<TrackInfo>(value)
        .map_err(|_| ChainStateError::FailedDecode)?;
    Ok(account_info)
}
