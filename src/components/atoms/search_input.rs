use dioxus::prelude::*;
use crate::components::atoms::{Icon, IconButton, Search};
use super::dropdown::ElementSize;
#[derive(PartialEq, Props, Clone)]
pub struct SearchInputProps {
    message: String,
    placeholder: String,
    #[props(!optional)]
    error: Option<String>,
    #[props(default = ElementSize::Medium)]
    size: ElementSize,
    label: Option<String>,
    on_input: EventHandler<FormEvent>,
    on_keypress: EventHandler<KeyboardEvent>,
    on_click: EventHandler<MouseEvent>,
}
pub fn SearchInput(props: SearchInputProps) -> Element {
    let input_error_container = if let Some(_) = props.error {
        "input--error-container"
    } else {
        ""
    };
    let size = match props.size {
        ElementSize::Big => "input-wrapper__container--big",
        ElementSize::Medium => "input-wrapper__container--medium",
        ElementSize::Small => "input-wrapper__container--small",
    };
    let mut is_active = use_signal::<bool>(|| false);
    rsx!(
        section {
            class: "input__wrapper input__wrapper--search",
            class: if is_active() { "input__wrapper--active" },
            if let Some(value) = props.label {
                label { class: "input__label", "{value}" }
            }
            div { class: "input-wrapper {size} {input_error_container}",
                input {
                    r#type: "text",
                    class: "input",
                    value: props.message,
                    placeholder: "{props.placeholder}",
                    oninput: move |event| props.on_input.call(event),
                    onkeypress: move |event| props.on_keypress.call(event)
                }
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
            if let Some(error) = props.error {
                div { class: "input--error", "{error}" }
            }
        }
    )
}
