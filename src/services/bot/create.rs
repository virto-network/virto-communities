use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::components::atoms::attach::AttachError;

#[derive(Serialize, Deserialize, Debug)]
pub struct CommunitySpace {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub industry: String,
}

#[derive(Deserialize)]
pub struct CommunityMatrixId {
    id: String,
}

impl CommunityMatrixId {
    pub fn get_id(self) -> String {
        self.id
    }
}

pub async fn create(community: CommunitySpace) -> Result<CommunityMatrixId, reqwest::Error> {
    let client = reqwest::Client::new();

    let response = client
        .post("https://bot-api.virto.app/room/create")
        .json(&community)
        .send()
        .await?
        .json::<CommunityMatrixId>()
        .await?;

    Ok(response)
}
