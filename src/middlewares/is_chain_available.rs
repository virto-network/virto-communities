use dioxus::router::hooks::use_navigator;
use dioxus_std::{i18n::use_i18, translate};
use web_sys::js_sys::Date;
use crate::{
    components::atoms::notification,
    hooks::{use_notification::use_notification, use_timestamp::use_timestamp},
};
const THRESHOLD: u64 = 1000 * 60;
pub fn is_chain_available() -> impl FnOnce() -> Result<(), &'static str> {
    move || {
        let i18 = use_i18();
        let timestamp = use_timestamp();
        let mut notification = use_notification();
        if Date::now() as u64 - timestamp.get().0 > THRESHOLD {
            notification
                .handle_warning(
                    &translate!(i18, "warnings.middleware.chain_unavailable"),
                );
            Err("Chain unavailable")
        } else {
            Ok(())
        }
    }
}
