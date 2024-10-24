use dioxus::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use crate::components::atoms::{Icon, IconButton, Search, WarningSign};
use super::dropdown::ElementSize;
#[derive(PartialEq, Clone)]
pub enum InputType {
    Text,
    Message,
    Search,
    Password,
    Date,
}
#[derive(PartialEq, Props, Clone)]
pub struct InputProps {
    #[props(default = InputType::Text)]
    itype: InputType,
    message: String,
    placeholder: String,
    #[props(!optional)]
    error: Option<String>,
    help: Option<String>,
    #[props(default = ElementSize::Medium)]
    size: ElementSize,
    label: Option<String>,
    #[props(default = false)]
    required: bool,
    #[props(default = 100)]
    maxlength: u8,
    left_text: Option<Element>,
    right_text: Option<Element>,
    on_input: EventHandler<FormEvent>,
    on_keypress: EventHandler<KeyboardEvent>,
    on_click: EventHandler<MouseEvent>,
}
pub fn Input(props: InputProps) -> Element {
    let mut input_ref = use_signal::<Option<Box<HtmlInputElement>>>(|| None);
    let input_error_container = if let Some(_) = props.error {
        "input--error-container"
    } else {
        ""
    };
    let input_type = match props.itype {
        InputType::Text => "text",
        InputType::Search => "text",
        InputType::Message => "text",
        InputType::Date => "date",
        InputType::Password => "password",
    };
    let size = match props.size {
        ElementSize::Big => "input-wrapper__container--big",
        ElementSize::Medium => "input-wrapper__container--medium",
        ElementSize::Small => "input-wrapper__container--small",
    };
    let is_search = if matches!(props.itype, InputType::Search) {
        "input__wrapper--search"
    } else {
        ""
    };
    let mut is_active = use_signal::<bool>(|| false);
    rsx!(
        section {
            class: "input__wrapper {is_search}",
            class: if is_active() { "input__wrapper--active" },
            if let Some(value) = props.label {
                label { class: "input__label", "{value}" }
            }
            div { class: "input-wrapper {size} {input_error_container}",
                {props.left_text},
                input {
                    r#type: "{input_type}",
                    class: "input",
                    onmounted: move |event| {
                        event
                            .data
                            .downcast::<web_sys::Element>()
                            .and_then(|element| element.clone().dyn_into::<HtmlInputElement>().ok())
                            .map(|html_element| input_ref.set(Some(Box::new(html_element.clone()))));
                        if input_type == "date" {
                            if let Some(input_element) = input_ref() {
                                input_element.set_type("text")
                            }
                        }
                    },
                    onfocus: move |_| {
                        if let Some(input_element) = input_ref() {
                            input_element.set_type(input_type)
                        }
                    },
                    onblur: move |_| {
                        if let Some(input_element) = input_ref() {
                            input_element.set_type("text")
                        }
                    },
                    value: props.message,
                    required: props.required,
                    maxlength: i64::from(props.maxlength),
                    placeholder: if props.required {
                        format!("{}*", props.placeholder)
                    } else {
                        format!("{}", props.placeholder)
                    },
                    oninput: move |event| props.on_input.call(event),
                    onkeypress: move |event| props.on_keypress.call(event)
                }
                {props.right_text},
                if matches!(props.itype, InputType::Search) {
                    IconButton {
                        class: "button--avatar bg--transparent",
                        size: props.size,
                        body: rsx! {
                            Icon {
                                icon: Search,
                                height: 26,
                                width: 26,
                                stroke_width: 1.5,
                                fill: "var(--text-secondary)"
                            }
                        },
                        on_click: move |_| {
                            is_active.toggle();
                        }
                    }
                }
            }
            if props.error.is_none() {
                if let Some(help) = props.help {
                    div { class: "input--help", "{help}" }
                }
            }
            if let Some(error) = props.error {
                div { class: "input--error",
                    Icon {
                        icon: WarningSign,
                        height: 24,
                        width: 24,
                        fill: "var(--state-destructive-active)"
                    }
                    "{error}"
                }
            }
        }
    )
}
