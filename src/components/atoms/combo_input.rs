use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

use crate::components::atoms::{dropdown::DropdownItem, Dropdown, Input};

use super::dropdown::ElementSize;

#[derive(PartialEq, Clone)]
pub struct ComboInputValue {
    pub dropdown: DropdownItem,
    pub input: String,
}

#[derive(PartialEq, Props, Clone)]
pub struct ComboInputProps {
    value: ComboInputValue,
    placeholder: String,
    #[props(default = ElementSize::Medium)]
    size: ElementSize,
    on_change: EventHandler<ComboInputValue>,
}

pub fn ComboInput(props: ComboInputProps) -> Element {
    let i18 = use_i18();

    let mut dropdown_value = use_signal::<DropdownItem>(|| props.value.dropdown);
    let mut input_value = use_signal::<String>(|| props.value.input.clone());
    let mut soon = use_signal::<bool>(|| {
        dropdown_value().key == "Email" || dropdown_value().key == "Telegram"
    });

    let mut items = vec![];
    let mut dropdown_options = vec![
        DropdownItem {
            key: "Wallet".to_string(),
            value: translate!(i18, "onboard.invite.form.wallet.label"),
        },
        // DropdownItem {
        //     key: "Email".to_string(),
        //     value: translate!(i18, "onboard.invite.form.email.label"),
        // },
        // DropdownItem {
        //     key: "Telegram".to_string(),
        //     value: translate!(i18, "onboard.invite.form.phone.label"),
        // },
    ];

    for account in dropdown_options.clone().into_iter() {
        items.push(rsx!(span {
            "{account.value}"
        }))
    }

    rsx!(
        div {
            class: "combo-input",
            class: if soon() { "combo-input--comming-soon" },
            Dropdown {
                class: "dropdown--left".to_string(),
                value: dropdown_value(),
                placeholder: translate!(i18, "header.cta.account"),
                size: props.size.clone(),
                default: None,
                on_change: move |event: usize| {
                    let to_assign = &dropdown_options[event];

                    if to_assign.value == "Email" || to_assign.value == "Telegram" {
                        soon.set(true)
                    } else  {
                        soon.set(false)
                    }

                    dropdown_value.set(to_assign.clone());
                    props.on_change.call(ComboInputValue { dropdown: dropdown_value(), input: input_value().clone() })
                },
                body: items
            }
            Input {
                message: props.value.input.clone(),
                size: props.size,
                placeholder: props.placeholder,
                error: None,
                on_input: move |event: Event<FormData>| {
                    input_value.set(event.value().clone());
                    props.on_change.call(ComboInputValue { dropdown: dropdown_value().clone(), input: input_value().clone() })
                },
                on_keypress: move |_| {},
                on_click: move |_| {},
            }
        }
    )
}
