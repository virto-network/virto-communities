use dioxus_std::{i18n::UseI18, translate};
use web_sys::js_sys::Date;

use crate::hooks::{use_notification::UseNotificationState, use_timestamp::UseTimestampState};

const THRESHOLD: u64 = 1000 * 60 * 10;
pub fn is_chain_available(
    i18: UseI18,
    timestamp: UseTimestampState,
    mut notification: UseNotificationState,
) -> impl FnOnce() -> Result<(), &'static str> {
    move || {
        log::info!("timestamp: {:?}", timestamp.get().0);
        log::info!("now: {:?}", Date::now());
        log::info!("rest: {:?}", Date::now() as u64 - timestamp.get().0);
        if Date::now() as u64 - timestamp.get().0 > THRESHOLD {
            log::warn!("Chain unavailable");
            notification.handle_warning(&translate!(i18, "warnings.title"), &translate!(i18, "warnings.middleware.chain_unavailable"));
            Err("Chain unavailable")
        } else {
            Ok(())
        }
    }
}
