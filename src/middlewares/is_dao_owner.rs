use dioxus::router::hooks::use_navigator;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::atoms::notification,
    hooks::{use_accounts::use_accounts, use_notification::use_notification},
};

pub fn is_dao_owner() -> impl FnOnce() -> Result<(), &'static str> {
    move || {
        let i18 = use_i18();
        let nav = use_navigator();
        let accounts = use_accounts();
        let mut notification = use_notification();

        if accounts.is_active_account_an_admin() {
            notification.handle_warning(&translate!(i18, "warnings.middleware.has_dao"));
            Err("User is DAO owner")
        } else {
            Ok(())
        }
    }
}