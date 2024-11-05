use crate::hooks::{use_accounts::UseAccountsState, use_notification::UseNotificationState};
use dioxus_std::{i18n::UseI18, translate};
pub fn is_signer_ready(
    i18: UseI18,
    accounts: UseAccountsState,
    mut notification: UseNotificationState,
) -> impl FnOnce() -> Result<(), &'static str> {
    move || {
        if accounts.get_account().is_none() {
            notification.handle_warning(&translate!(i18, "warnings.title"), &translate!(i18, "warnings.middleware.signer_not_found"));
            Err("Failed to get account to sign")
        } else {
            log::debug!("Signer is ready");
            Ok(())
        }
    }
}
