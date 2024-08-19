use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

use super::{use_accounts::use_accounts, use_session::use_session};

pub enum PjsError {
    ConnectionFailed,
    AccountsNotFound,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = globalThis, js_name = initExecutor)]
    pub fn init_executor();
}

const APP_NAME: &str = "Virto";

pub async fn use_connect_wallet() -> Result<(), PjsError> {
    let session = use_session();
    let mut accounts = use_accounts();
    let mut pjs = use_context::<Signal<Option<pjs::PjsExtension>>>();

    let mut vault = pjs::PjsExtension::connect(APP_NAME).await.map_err(|_| {
        if let Err(_) = session.persist_session_file("") {
            log::warn!("Failed to persist session")
        };

        PjsError::ConnectionFailed
    })?;

    init_executor();

    vault
        .fetch_accounts()
        .await
        .map_err(|_| PjsError::AccountsNotFound)?;

    let vault_accounts = vault.accounts();

    accounts.set(vault_accounts);
    pjs.set(Some(vault));

    Ok(())
}
