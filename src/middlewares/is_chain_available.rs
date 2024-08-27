use dioxus_std::{i18n::UseI18, translate};
use web_sys::js_sys::Date;

use crate::hooks::{use_notification::UseNotificationState, use_timestamp::UseTimestampState};

const THRESHOLD: u64 = 1000 * 60;
pub fn is_chain_available(
    i18: UseI18,
    timestamp: UseTimestampState,
    mut notification: UseNotificationState,
) -> impl FnOnce() -> Result<(), &'static str> {
    move || {
        if Date::now() as u64 - timestamp.get().0 > THRESHOLD {
            notification.handle_warning(&translate!(i18, "warnings.middleware.chain_unavailable"));
            Err("Chain unavailable")
        } else {
            Ok(())
        }
    }
}
