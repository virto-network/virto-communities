use crate::hooks::{use_accounts::UseAccountsState, use_notification::UseNotificationState};
use dioxus_i18n::t;
pub fn is_dao_owner(
    accounts: UseAccountsState,
    mut notification: UseNotificationState,
) -> impl FnOnce() -> Result<(), &'static str> {
    move || {
        if accounts.is_active_account_an_admin() {
            dioxus::logger::tracing::warn!("User is DAO owner");
            notification.handle_warning(&t!("warnings-title"), &t!("warnings-middleware-has_dao"));
            Err("User is DAO owner")
        } else {
            Ok(())
        }
    }
}
