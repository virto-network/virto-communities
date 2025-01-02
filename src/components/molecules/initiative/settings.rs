use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::atoms::{
    dropdown::{DropdownItem, ElementSize},
    Dropdown,
};
#[component]
pub fn InitiativeSettings() -> Element {
    
    let mut dropdown_value = use_signal::<Option<DropdownItem>>(|| None);
    let mut items = vec![];
    let dropdown_options = vec![
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
                    placeholder: t!("header-cta-account"),
                    size: ElementSize::Small,
                    default: None,
                    on_change: move |event: usize| {
                        let dropdown_options = [
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
