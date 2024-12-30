use dioxus::{logger::tracing::warn, prelude::*};
use wasm_bindgen::prelude::*;

use super::{use_accounts::Account, use_session::use_session};

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
    let mut accounts = consume_context::<Signal<Vec<Account>>>();
    let mut pjs = use_context::<Signal<Option<pjs::PjsExtension>>>();

    let mut vault = pjs::PjsExtension::connect(APP_NAME).await.map_err(|_| {
        if session.persist_session_file("").is_err() {
            warn!("Failed to persist session")
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
