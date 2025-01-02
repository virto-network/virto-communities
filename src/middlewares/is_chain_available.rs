use dioxus::logger::tracing::{debug, warn};
use dioxus_i18n::t;
use web_sys::js_sys::Date;

use crate::hooks::{use_notification::UseNotificationState, use_timestamp::UseTimestampState};

const THRESHOLD: u64 = 1000 * 60 * 10;
pub fn is_chain_available(
    timestamp: UseTimestampState,
    mut notification: UseNotificationState,
) -> impl FnOnce() -> Result<(), &'static str> {
    move || {
        debug!("timestamp: {:?}", timestamp.get().0);
        debug!("now: {:?}", Date::now());
        debug!("rest: {:?}", Date::now() as u64 - timestamp.get().0);
        if Date::now() as u64 - timestamp.get().0 > THRESHOLD {
            warn!("Chain unavailable");
            notification.handle_warning(&t!("warnings-title"), &t!("warnings-middleware-chain_unavailable"));
            Err("Chain unavailable")
        } else {
            Ok(())
        }
    }
}
