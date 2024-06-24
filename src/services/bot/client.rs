use std::ops::Deref;

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

    pub async fn get_by_alias(&self, alias: &str) -> Result<CommunitySpace, reqwest::Error> {
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

    pub async fn upload(&self, content: Vec<u8>, name: String) -> Result<String, reqwest::Error> {
        let path = format!("{}/room/upload", self.base_path);
        let infered_type = infer::get(content.deref()).expect("Should infer the content type");

        let part = reqwest::multipart::Part::stream(content)
            .file_name(name.clone())
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
}
