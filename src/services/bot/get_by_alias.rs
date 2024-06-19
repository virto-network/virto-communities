use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::components::atoms::attach::AttachError;

use super::create::CommunitySpace;

pub async fn get_by_alias(alias: &str) -> Result<CommunitySpace, reqwest::Error> {
    let client = reqwest::Client::new();

    let response = client
        .get(format!("https://bot-api.virto.app/room/alias/{}", alias))
        .send()
        .await?
        .json::<CommunitySpace>()
        .await?;

    Ok(response)
}
