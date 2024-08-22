use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use futures_util::TryFutureExt;
use super::{use_accounts::use_accounts, use_notification::use_notification};
pub enum PjsError {
    ConnectionFailed,
    AccountsNotFound,
}
pub fn use_connect_wallet() {
    let i18 = use_i18();
    let mut accounts = use_accounts();
    let mut notification = use_notification();
    let mut pjs = use_context::<Signal<Option<pjs::PjsExtension>>>();
    use_coroutine(|_: UnboundedReceiver<()>| {
        async move {
            let mut vault = pjs::PjsExtension::connect("virto")
                .await
                .map_err(|_| PjsError::ConnectionFailed)?;
            vault.fetch_accounts().await.map_err(|_| PjsError::AccountsNotFound)?;
            let vault_accounts = vault.accounts();
            accounts.set(vault_accounts);
            pjs.set(Some(vault));
            Ok::<(), PjsError>(())
        }
            .unwrap_or_else(move |e: PjsError| {
                match e {
                    PjsError::ConnectionFailed => {
                        notification
                            .handle_error(
                                &translate!(i18, "errors.wallet.connection_failed"),
                            )
                    }
                    PjsError::AccountsNotFound => {
                        notification
                            .handle_error(
                                &translate!(i18, "errors.wallet.accounts_not_found"),
                            );
                    }
                };
            })
    });
}
