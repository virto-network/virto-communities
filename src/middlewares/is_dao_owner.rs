use crate::hooks::{use_accounts::UseAccountsState, use_notification::UseNotificationState};
use dioxus::logger::tracing::warn;
use dioxus_i18n::t;
pub fn is_dao_owner(
    accounts: UseAccountsState,
    mut notification: UseNotificationState,
) -> impl FnOnce() -> Result<(), &'static str> {
    move || {
        if accounts.is_active_account_an_admin() {
            warn!("User is DAO owner");
            Err("User is DAO owner")
        } else {
            Ok(())
        }
    }
}
