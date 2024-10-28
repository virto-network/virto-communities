use crate::components::atoms::Input;
use dioxus::prelude::*;

use super::dropdown::ElementSize;
#[derive(PartialEq, Debug, Clone, Default)]
pub struct AutocompleteItem {
    pub key: String,
    pub value: String,
}
#[derive(PartialEq, Props, Clone)]
pub struct AutocompleteProps {
    value: Option<AutocompleteItem>,
    label: Option<String>,
    on_change: EventHandler<usize>,
    on_input: EventHandler<FormEvent>,
    #[props(!optional)]
    default: Option<String>,
    #[props(default = false)]
    disabled: bool,
    #[props(default = false)]
    reverse: bool,
    #[props(default = "".to_string())]
    class: String,
    placeholder: String,
    help: Option<String>,
    left_icon: Option<Element>,
    #[props(default = ElementSize::Medium)]
    size: ElementSize,
    body: Vec<Element>,
    add_element: Option<Element>,
}
pub fn Autocomplete(props: AutocompleteProps) -> Element {
    let mut is_active = use_signal::<bool>(|| false);
    let mut search = use_signal::<String>(|| String::new());

    let disabled = if props.disabled {
        "button--disabled"
    } else {
        ""
    };
    let placeholder = if let None = props.value {
        "autocomplete__placeholder"
    } else {
        ""
    };
    let size = match props.size {
        ElementSize::Big => "autocomplete__container--big",
        ElementSize::Medium => "autocomplete__container--medium",
        ElementSize::Small => "autocomplete__container--small",
    };
    rsx!(
        section { class: "autocomplete {props.class}",
            if let Some(value) = props.label {
                label { class: "input__label",
                    "{value}"
                }
            }
            if is_active() {
                Input {
                    message: search(),
                    placeholder: "".to_string(),
                    size: ElementSize::Small,
                    autofocus: true,
                    focus: is_active(),
                    error: None,
                    on_input: move |event: Event<FormData>| {
                        search.set(event.value());
                        props.on_input.call(event);
                    },
                    on_keypress: move |_| {},
                    on_click: move |_| {},
                    on_focus: move |_| {
                        is_active.set(true)
                    },
                    on_blur: move |_| {
                        // is_active.set(false)
                    }
                }
            } else {
                div { class: "dropdown__container {size}",
                    button {
                        class: "dropdown__wrapper {disabled}",
                        disabled: props.disabled,
                        onclick: move |_| {
                            if !props.disabled {
                                is_active.toggle();
                            }
                        },
                        onblur: move |_| {
                            // is_active.set(false);
                        },
                        {props.left_icon},
                        span { class: "dropdown__content",
                            span { class: "dropdown__value {placeholder}",
                                match props.value {
                                    Some(v) => {v.value}.to_string(),
                                    None => props.placeholder
                                }
                            }
                        }
                    }
                }
            }
            if is_active() {
                {
                    rsx!(
                        ul {
                            class : "autocomplete__list",
                            class : if props.reverse { "autocomplete__list--reverse" },
                            {
                                props.body.into_iter().enumerate().map(| (index, item) | {
                                    rsx!(
                                        li {
                                            class : "autocomplete__item",
                                            onclick : move | _ | {
                                                log::info!("click item");
                                                is_active.toggle();
                                                props.on_change.call(index);
                                            },
                                            { item }
                                        }
                                    )
                                })
                            }
                            if let Some(help) = props.add_element {
                                div { class: "autocomplete__item", {help} }
                            }
                        }
                    )
                }
            }
        }
    )
}
