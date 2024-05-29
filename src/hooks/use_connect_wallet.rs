use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use futures_util::TryFutureExt;
use libwallet::Wallet as LibWallet;

use crate::services::pjs::Pjs;

use super::{use_accounts::use_accounts, use_notification::use_notification};

pub type Wallet = LibWallet<Pjs>;

pub enum PjsError {
    ConnectionFailed,
    AccountsNotFound,
}

pub fn use_connect_wallet() {
    let i18 = use_i18();
    let mut accounts = use_accounts();
    let mut notification = use_notification();
    let mut pjs = use_context::<Signal<Option<Pjs>>>();

    use_coroutine(|_: UnboundedReceiver<()>| {
        async move {
            let mut vault = Pjs::connect("virto")
                .await
                .map_err(|_| PjsError::ConnectionFailed)?;

            let vault_accounts = vault
                .list_accounts()
                .await
                .map_err(|_| PjsError::AccountsNotFound)?;

            accounts.set(vault_accounts);

            pjs.set(Some(vault));

            Ok::<(), PjsError>(())
        }
        .unwrap_or_else(move |e: PjsError| {
            match e {
                PjsError::ConnectionFailed => {
                    notification.handle_error(&translate!(i18, "errors.wallet.connection_failed"))
                }
                PjsError::AccountsNotFound => {
                    notification.handle_error(&translate!(i18, "errors.wallet.accounts_not_found"));
                }
            };
        })
    });
}
