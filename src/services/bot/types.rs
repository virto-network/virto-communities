use serde::{Deserialize, Serialize};

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

#[derive(Deserialize)]
pub struct Uri {
    uri: String,
}

impl Uri {
    pub fn get(self) -> String {
        self.uri
    }
}
