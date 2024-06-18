use std::ops::Deref;

use serde::Deserialize;

use crate::components::atoms::attach::AttachError;

#[derive(Deserialize)]
pub struct Uri {
    uri: String,
}

pub async fn upload(content: Vec<u8>, name: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let infered_type = infer::get(content.deref()).unwrap();

    let part = reqwest::multipart::Part::stream(content)
        .file_name(name.clone())
        .mime_str(&infered_type.to_string())
        .expect("Couldn't set MIME type");

    let form = reqwest::multipart::Form::new().part("file", part);

    let response = client
        .post("https://bot-api.virto.app/room/upload")
        .multipart(form)
        .send()
        .await?
        .json::<Uri>()
        .await?;

    Ok(response.uri)
}
