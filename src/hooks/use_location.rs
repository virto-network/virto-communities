use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use gloo::storage::{LocalStorage, Storage};
use wasm_bindgen_futures::spawn_local;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Location {
    pub city: String,
    pub country: String,
}

#[derive(Clone, Debug)]
pub enum LocationError {
    FetchError,
    StorageError,
}

pub fn use_location() -> UseLocationState {
    let mut location = use_signal(|| None);
    let mut error = use_signal(|| None);

    use_coroutine(move |_:UnboundedReceiver<()>| async move {
        if let Some(stored_location) = LocalStorage::get("user_location").ok() {
                    location.set(Some(stored_location));
                    log::info!("{}", "Location loaded from storage");
                } else {
                    match fetch_and_store_location().await {
                        Ok(loc) => {
                            location.set(Some(loc.clone()));
                            error.set(None);
                            log::info!("{}", &format!("Location fetched: {}, {}", loc.city, loc.country));
                        }
                        Err(err) => {
                            location.set(None);
                            error.set(Some(err.clone()));
                            log::error!("{}", &format!("Error fetching location: {:?}", err));
                        }
                    }
                }
    });

    UseLocationState { location, error }
}

#[derive(Clone, Copy)]
pub struct UseLocationState {
    location: Signal<Option<Location>>,
    error: Signal<Option<LocationError>>,
}

impl UseLocationState {
    pub fn get_location(&self) -> Option<Location> {
        self.location.read().clone()
    }

    pub fn get_error(&self) -> Option<LocationError> {
        self.error.read().clone()
    }

    pub fn is_loading(&self) -> bool {
        self.location.read().is_none() && self.error.read().is_none()
    }
}

async fn fetch_and_store_location() -> Result<Location, LocationError> {
    let client = reqwest::Client::new();
    let res = client
        .get("https://ipapi.co/json/")
        .send()
        .await
        .map_err(|_| LocationError::FetchError)?;

    let location: Location = res.json().await.map_err(|_| LocationError::FetchError)?;

    LocalStorage::set("user_location", &location).map_err(|_| LocationError::StorageError)?;

    Ok(location)
}