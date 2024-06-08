use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::components::atoms::attach::AttachError;

use super::create::CommunitySpace;

pub async fn get_by_id(id: &str) -> Result<CommunitySpace, reqwest::Error> {
    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://127.0.0.1:8000/room/{}", id))
        .send()
        .await?
        .json::<CommunitySpace>()
        .await?;

    Ok(response)
}
