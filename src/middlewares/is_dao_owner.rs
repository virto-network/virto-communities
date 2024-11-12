use crate::hooks::{use_accounts::UseAccountsState, use_notification::UseNotificationState};
use dioxus_std::{i18n::UseI18, translate};
pub fn is_dao_owner(
    i18: UseI18,
    accounts: UseAccountsState,
    mut notification: UseNotificationState,
) -> impl FnOnce() -> Result<(), &'static str> {
    move || {
        if accounts.is_active_account_an_admin() {
            log::warn!("User is DAO owner");
            notification.handle_warning(&translate!(i18, "warnings.title"), &translate!(i18, "warnings.middleware.has_dao"));
            Err("User is DAO owner")
        } else {
            Ok(())
        }
    }
}
