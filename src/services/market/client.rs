use super::types::{Response, Tokens};

#[derive(Clone)]
pub struct MarketClient {
    base_path: String,
    client: reqwest::Client,
}

impl MarketClient {
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: base_path.to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_price_by_token(&self, token: Tokens) -> Result<f64, reqwest::Error> {
        let path = format!(
            "{}/his/coin/trend?code={}&type=s",
            self.base_path,
            token.name(),
        );

        let response = self.client.get(path.clone()).send().await?.text().await?;
        let response =
            serde_json::from_str::<Response>(&response).expect("should be a json response");

        let price = response.get_price();

        Ok(price)
    }
}
