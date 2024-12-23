use dioxus_i18n::t;
use web_sys::js_sys::Date;

use crate::hooks::{use_notification::UseNotificationState, use_timestamp::UseTimestampState};

const THRESHOLD: u64 = 1000 * 60 * 10;
pub fn is_chain_available(
    timestamp: UseTimestampState,
    mut notification: UseNotificationState,
) -> impl FnOnce() -> Result<(), &'static str> {
    move || {
        dioxus::logger::tracing::info!("timestamp: {:?}", timestamp.get().0);
        dioxus::logger::tracing::info!("now: {:?}", Date::now());
        dioxus::logger::tracing::info!("rest: {:?}", Date::now() as u64 - timestamp.get().0);
        if Date::now() as u64 - timestamp.get().0 > THRESHOLD {
            dioxus::logger::tracing::warn!("Chain unavailable");
            notification.handle_warning(&t!("warnings-title"), &t!("warnings-middleware-chain_unavailable"));
            Err("Chain unavailable")
        } else {
            Ok(())
        }
    }
}
