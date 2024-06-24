use std::ops::Deref;

use dioxus::prelude::*;

use crate::services::bot::client::SpacesClient;

pub fn use_spaces_client() -> UseSpacesClient {
    let client = consume_context::<Signal<SpacesClient>>();

    use_hook(move || UseSpacesClient { client })
}

#[derive(Clone, Copy)]
pub struct UseSpacesClient {
    pub client: Signal<SpacesClient>,
}

impl UseSpacesClient {
    pub fn get(&self) -> SpacesClient {
        self.client.read().clone()
    }
}
