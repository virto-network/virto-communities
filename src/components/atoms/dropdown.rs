use dioxus::prelude::*;

use crate::components::atoms::{button::Variant as ButtonVariant, Arrow, Button, Icon};

#[derive(PartialEq, Clone, Default)]
pub struct DropdownItem {
    pub key: String,
    pub value: String,
}

#[derive(PartialEq, Props, Clone)]
pub struct DropdownProps {
    value: Option<DropdownItem>,
    items: Vec<DropdownItem>,
    label: Option<String>,
    on_change: EventHandler<DropdownItem>,
    #[props(!optional)]
    default: Option<String>,
    #[props(default = false)]
    disabled: bool,
}

pub fn Dropdown(props: DropdownProps) -> Element {
    let mut is_active = use_signal(|| false);

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

    rsx!(
        section {
            class: "dropdown__wrapper",
            if let Some(value) = props.label {
                label { class: "dropdown__label", "{value}" }
            }
            button {
                class: "dropdown {disabled}",
                disabled: props.disabled,
                onclick: move |_| {
                    if !props.disabled {
                        is_active.toggle()
                    }
                },
                span {
                    class: "dropdown__content",
                    span {
                        class: "{placeholder}",
                        match props.value {
                            Some(v) => {v.value}.to_string(),
                            None => "Selecciona...".to_string()
                        }
                    }
                    Icon {
                        class: if is_active() { "rotate-180" } else { "rotate-0" },
                        icon: Arrow,
                        height: 24,
                        width: 24,
                        stroke_width: 2,
                        stroke: "var(--text-1)"
                    }
                }
            }
            if is_active() {
                ul {
                    class: "dropdown__list",
                    for item in props.items.into_iter() {
                        li {
                            class: "dropdown__item",
                            Button {
                                text: item.value.clone(),
                                variant: ButtonVariant::Tertiary,
                                on_click: move |_| {
                                    is_active.toggle();
                                    props.on_change.call(item.clone());
                                },
                                status: None
                            }
                        }
                    }
                }
            }
        }
    )
}
