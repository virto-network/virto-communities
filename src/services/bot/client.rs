use crate::hooks::use_initiative::{
    InitiativeData, InitiativeHistory, InitiativeVoteData,
};
use super::types::{CommunityMatrixId, CommunitySpace, Uri};
#[derive(Clone)]
pub struct SpacesClient {
    base_path: String,
    client: reqwest::Client,
}
impl SpacesClient {
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: base_path.to_string(),
            client: reqwest::Client::new(),
        }
    }
    pub async fn create(
        &self,
        community: CommunitySpace,
    ) -> Result<CommunityMatrixId, reqwest::Error> {
        let path = format!("{}/room/create", self.base_path);
        let response = self
            .client
            .post(path)
            .json(&community)
            .send()
            .await?
            .json::<CommunityMatrixId>()
            .await?;
        Ok(response)
    }
    pub async fn get_by_alias(
        &self,
        alias: &str,
    ) -> Result<CommunitySpace, reqwest::Error> {
        let path = format!("{}/room/alias/{}", self.base_path, alias);
        let response = self
            .client
            .get(path)
            .send()
            .await?
            .json::<CommunitySpace>()
            .await?;
        Ok(response)
    }
    pub async fn get_by_id(&self, id: &str) -> Result<CommunitySpace, reqwest::Error> {
        let path = format!("{}/room/id/{}", self.base_path, id);
        let response = self
            .client
            .get(path)
            .send()
            .await?
            .json::<CommunitySpace>()
            .await?;
        Ok(response)
    }
    pub async fn upload(
        &self,
        content: &[u8],
        name: &str,
    ) -> Result<String, reqwest::Error> {
        let path = format!("{}/room/upload", self.base_path);
        let infered_type = infer::get(content).expect("Should infer the content type");
        let part = reqwest::multipart::Part::stream(content.to_vec())
            .file_name(name.to_string())
            .mime_str(&infered_type.to_string())
            .expect("Couldn't set MIME type");
        let form = reqwest::multipart::Form::new().part("file", part);
        let response = self
            .client
            .post(path)
            .multipart(form)
            .send()
            .await?
            .json::<Uri>()
            .await?;
        Ok(response.get())
    }
    pub async fn create_initiative(
        &self,
        initiative: InitiativeData,
    ) -> Result<CommunityMatrixId, reqwest::Error> {
        let path = format!("{}/initiative/create", self.base_path);
        let response = self
            .client
            .post(path)
            .json(&initiative)
            .send()
            .await?
            .json::<CommunityMatrixId>()
            .await?;
        Ok(response)
    }
    pub async fn vote_initiative(
        &self,
        vote: InitiativeVoteData,
    ) -> Result<(), reqwest::Error> {
        let path = format!("{}/initiative/vote", self.base_path);
        self.client.post(path).json(&vote).send().await?;
        Ok(())
    }
    pub async fn get_initiative_by_id(
        &self,
        id: &str,
    ) -> Result<InitiativeHistory, reqwest::Error> {
        let path = format!("{}/initiative/{}", self.base_path, id);
        let response = self
            .client
            .get(path)
            .send()
            .await?
            .json::<InitiativeHistory>()
            .await?;
        Ok(response)
    }
}
