use dioxus::prelude::*;
use crate::{
    components::atoms::{dropdown::ElementSize, IconButton, Icon, AddPlus},
    hooks::{use_notification::use_notification, use_our_navigator::use_our_navigator, use_timestamp::use_timestamp, use_accounts::use_accounts},
    middlewares::{is_chain_available::is_chain_available, is_dao_owner::is_dao_owner},
    
};
use dioxus_std::i18n::use_i18;

#[derive(PartialEq, Props, Clone)]
pub struct CreateDaoButtonProps {
    #[props(default = "button--avatar desktop ailen".to_string())]
    pub class: String,
}

#[component]
pub fn CreateDaoButton(props: CreateDaoButtonProps) -> Element {
    let i18 = use_i18();
    let nav = use_our_navigator();
    let notification = use_notification();
    let timestamp = use_timestamp();
    let accounts = use_accounts();

    rsx! {
        IconButton {
            class: "{props.class}",
            size: ElementSize::Medium,
            body: rsx! {
                Icon {
                    icon: AddPlus,
                    height: 26,
                    width: 26,
                    stroke_width: 1.5,
                    fill: "var(--fill-00)"
                }
            },
            on_click: move |_| {
                nav.push(
                    vec![
                        Box::new(is_chain_available(i18, timestamp, notification)),
                        Box::new(is_dao_owner(i18, accounts, notification)),
                    ],
                    "/onboarding",
                );
            }
        }
    }
}
