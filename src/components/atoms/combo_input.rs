use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use crate::components::atoms::{
    dropdown::DropdownItem, input::InputType, Dropdown, Input,
};
use super::dropdown::ElementSize;
#[derive(PartialEq, Clone, Debug)]
pub enum ComboInputOption {
    Dropdown(DropdownItem),
    Date(String),
    None,
}
#[derive(PartialEq, Clone, Debug)]
pub struct ComboInputValue {
    pub option: ComboInputOption,
    pub input: String,
}
#[derive(PartialEq, Props, Clone)]
pub struct ComboInputProps {
    #[props(default = "".to_string())]
    class: String,
    value: ComboInputValue,
    placeholder: String,
    error: Option<String>,
    options: Option<Vec<DropdownItem>>,
    #[props(default = ElementSize::Medium)]
    size: ElementSize,
    left_text: Option<Element>,
    right_text: Option<Element>,
    on_change: EventHandler<ComboInputValue>,
}
pub fn ComboInput(props: ComboInputProps) -> Element {
    let i18 = use_i18();
    let mut option_value = use_signal(|| props.value.option.clone());
    let mut input_value = use_signal::<String>(|| props.value.input.clone());
    let mut items = vec![];
    let dropdown_options = use_signal::<
        Vec<DropdownItem>,
    >(|| {
        let Some(options) = props.options else {
            return vec![
                DropdownItem {
                    key: "Wallet".to_string(),
                    value: translate!(i18, "onboard.invite.form.wallet.label"),
                },
            ];
        };
        options
    });
    for account in dropdown_options().clone().into_iter() {
        items.push(rsx!(
            span { "{account.value}" }
        ))
    }
    rsx!(
        div { class: "combo-input {props.class}",
            match option_value() {
                ComboInputOption::Date(value) => rsx!(
                    Input {
                        message: value,
                        size: props.size.clone(),
                        itype: InputType::Date,
                        placeholder: props.placeholder.clone(),
                        error: None,
                        on_input: move |event: Event<FormData>| {
                            option_value.set(ComboInputOption::Date(event.value().clone()));
                            props.on_change.call(ComboInputValue { option: ComboInputOption::Date(event.value().clone()), input: input_value().clone() })
                        },
                        on_keypress: move |_| {},
                        on_click: move |_| {},
                    }
                ),
                ComboInputOption::Dropdown(value) => rsx!(
                    Dropdown {
                            class: "dropdown--left".to_string(),
                            value: value,
                            placeholder: translate!(i18, "header.cta.account"),
                            size: props.size.clone(),
                            default: None,
                            on_change: move |event: usize| {
                                let to_assign = &dropdown_options()[event];
                                option_value.set(ComboInputOption::Dropdown(to_assign.clone()));
                                props.on_change.call(ComboInputValue { option: ComboInputOption::Dropdown(to_assign.clone()), input: input_value().clone() })
                            },
                            body: items
                    }
                ),
                ComboInputOption::None => {
                    rsx!()
                }
            },
            Input {
                message: props.value.input.clone(),
                size: props.size,
                placeholder: props.placeholder,
                error: None,
                right_text: props.right_text,
                on_input: move |event: Event<FormData>| {
                    input_value.set(event.value().clone());
                    props
                        .on_change
                        .call(ComboInputValue {
                            option: option_value(),
                            input: input_value().clone(),
                        })
                },
                on_keypress: move |_| {},
                on_click: move |_| {}
            }
        }
    )
}
