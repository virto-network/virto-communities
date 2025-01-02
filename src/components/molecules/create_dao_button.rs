use dioxus::prelude::*;
use dioxus_i18n::t;
use crate::{
    components::atoms::{dropdown::ElementSize, IconButton, Icon, AddPlus},
    hooks::{use_notification::use_notification, use_our_navigator::use_our_navigator, use_timestamp::use_timestamp, use_accounts::use_accounts},
    middlewares::{is_chain_available::is_chain_available, is_dao_owner::is_dao_owner},
};

#[derive(PartialEq, Props, Clone)]
pub struct CreateDaoButtonProps {
    #[props(default = "button--avatar desktop".to_string())]
    pub class: String,
}

#[component]
pub fn CreateDaoButton(props: CreateDaoButtonProps) -> Element {
    let nav = use_our_navigator();
    let notification = use_notification();
    let timestamp = use_timestamp();
    let accounts = use_accounts();
    let mut show_tooltip = use_signal(|| false);

    let handle_click = move |_| {
        nav.push(
            vec![
                Box::new(is_dao_owner(accounts.clone(), notification.clone())),
            ],
            "/onboarding",
        );
    };

    let handle_mouse_enter = move |_| {
        let chain_available = is_chain_available(timestamp.clone(), notification.clone())();
        let is_owner = is_dao_owner(accounts.clone(), notification.clone())();
        
        if chain_available.is_ok() && is_owner.is_err() {
            show_tooltip.set(true);
        }
    };

    let handle_mouse_leave = move |_| {
        show_tooltip.set(false);
    };

    rsx! {
        div {
            class: "create-dao-button-wrapper",
            onmouseenter: handle_mouse_enter,
            onmouseleave: handle_mouse_leave,
            IconButton {
                class: "{props.class}",
                size: ElementSize::Medium,
                on_click: handle_click,
                body: rsx! {
                    Icon {
                        icon: AddPlus,
                        height: 26,
                        width: 26,
                        stroke_width: 1.5,
                        fill: "var(--fill-00)"
                    }
                }
            }
            div {
                class: "create-dao-tooltip-container",
                div {
                    class: "create-dao-tooltip",
                    class: if (*show_tooltip)() { "create-dao-tooltip--visible" } else { "" },
                    { t!("warnings-middleware-has_dao") },
                }
            }
        }
    }
}

