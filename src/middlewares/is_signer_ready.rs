use crate::hooks::{use_accounts::UseAccountsState, use_notification::UseNotificationState};
use dioxus_i18n::t;
pub fn is_signer_ready(
    accounts: UseAccountsState,
    mut notification: UseNotificationState,
) -> impl FnOnce() -> Result<(), &'static str> {
    move || {
        if accounts.get_account().is_none() {
            notification.handle_warning(&t!("warnings-title"), &t!("warnings-middleware-signer_not_found"));
            Err("Failed to get account to sign")
        } else {
            dioxus::logger::tracing::debug!("Signer is ready");
            Ok(())
        }
    }
}
