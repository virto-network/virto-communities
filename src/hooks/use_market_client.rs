use dioxus::prelude::*;

use crate::services::market::client::MarketClient;

pub fn use_market_client() -> UseMarketClient {
    let client = consume_context::<Signal<MarketClient>>();

    use_hook(move || UseMarketClient { client })
}

#[derive(Clone, Copy)]
pub struct UseMarketClient {
    pub client: Signal<MarketClient>,
}

impl UseMarketClient {
    pub fn get(&self) -> MarketClient {
        self.client.read().clone()
    }
}
