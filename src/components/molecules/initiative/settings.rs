use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use crate::{
    components::atoms::{
        dropdown::{DropdownItem, ElementSize},
        Dropdown,
    },
    hooks::{
        use_notification::use_notification, use_onboard::use_onboard,
        use_spaces_client::use_spaces_client,
    },
};
#[component]
pub fn InitiativeSettings() -> Element {
    let i18 = use_i18();
    let mut onboard = use_onboard();
    let mut notification = use_notification();
    let spaces_client = use_spaces_client();
    let mut dropdown_value = use_signal::<Option<DropdownItem>>(|| None);
    let mut items = vec![];
    let mut dropdown_options = vec![
        DropdownItem {
            key: "Admin".to_string(),
            value: "Admin".to_string(),
        },
        DropdownItem {
            key: "Community".to_string(),
            value: "Community".to_string(),
        },
    ];
    for account in dropdown_options.clone().into_iter() {
        items.push(rsx!(
            span { "{account.value}" }
        ))
    }
    rsx!(
        div { class: "form__inputs form__inputs--initiative",
            div { class: "form__input form__input--initiative",
                div { class: "form__input__info",
                    span { class: "form__input__info__title", "Origin" }
                    p { class: "form__input__info__description", "Lorem Ipsum Dolor" }
                }
                Dropdown {
                    class: "header__wallet dropdown--left".to_string(),
                    value: dropdown_value(),
                    placeholder: translate!(i18, "header.cta.account"),
                    size: ElementSize::Small,
                    default: None,
                    on_change: move |event: usize| {
                        let mut dropdown_options = vec![
                            DropdownItem {
                                key: "Admin".to_string(),
                                value: "Admin".to_string(),
                            },
                            DropdownItem {
                                key: "Community".to_string(),
                                value: "Community".to_string(),
                            },
                        ];
                        let x = &dropdown_options[event];
                        dropdown_value.set(Some(x.clone()))
                    },
                    body: items
                }
            }
        }
    )
}
