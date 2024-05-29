use dioxus::prelude::*;

use crate::components::atoms::{Arrow, Icon};

#[derive(PartialEq, Debug, Clone, Default)]
pub struct DropdownItem {
    pub key: String,
    pub value: String,
}

#[derive(PartialEq, Clone)]
pub enum ElementSize {
    Big,
    Medium,
    Small,
}

#[derive(PartialEq, Props, Clone)]
pub struct DropdownProps {
    value: Option<DropdownItem>,
    label: Option<String>,
    on_change: EventHandler<usize>,
    #[props(!optional)]
    default: Option<String>,
    #[props(default = false)]
    disabled: bool,
    #[props(default = "".to_string())]
    class: String,
    placeholder: String,
    #[props(default = ElementSize::Medium)]
    size: ElementSize,
    body: Vec<Element>,
}

pub fn Dropdown(props: DropdownProps) -> Element {
    let mut is_active = use_signal::<bool>(|| false);
    let disabled = if props.disabled {
        "button--disabled"
    } else {
        ""
    };

    let placeholder = if let None = props.value {
        "dropdown__placeholder"
    } else {
        ""
    };

    let size = match props.size {
        ElementSize::Big => "dropdown__container--big",
        ElementSize::Medium => "dropdown__container--medium",
        ElementSize::Small => "dropdown__container--small",
    };

    rsx!(
        section {
            class: "dropdown {props.class}",
            if let Some(value) = props.label {
                label { class: "dropdown__label", "{value}" }
            }
            div { class: "dropdown__container {size}",
                button {
                    class: "dropdown__wrapper {disabled}",
                    disabled: props.disabled,
                    onclick: move |_| {
                        if !props.disabled {
                            is_active.toggle();
                        }
                    },
                    span {
                        class: "dropdown__content",
                        span {
                            class: "dropdown__value {placeholder}",
                            match props.value {
                                Some(v) => {v.value}.to_string(),
                                None => props.placeholder
                            }
                        }
                        Icon {
                            class: if is_active() { "rotate-180" } else { "rotate-0" },
                            icon: Arrow,
                            height: 24,
                            width: 24,
                            stroke_width: 2,
                            stroke: "var(--text-primary, #12352b)"
                        }
                    }
                }
                if is_active() {
                    {rsx!(
                        ul {
                            class: "dropdown__list",
                            {
                                props.body.into_iter().enumerate().map(|(index, item)| {
                                    rsx!(
                                    li {
                                        class: "dropdown__item",
                                        onclick: move |_| {
                                            is_active.toggle();
                                            props.on_change.call(index)
                                        },
                                        {item}
                                    })
                                })
                            }
                        }
                    )}
                }
            }
        }
    )
}
