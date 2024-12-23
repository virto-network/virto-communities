use super::dropdown::ElementSize;
use crate::components::atoms::icons::Close;
use crate::components::atoms::{Icon, IconButton, WarningSign};
use dioxus::prelude::*;
#[derive(PartialEq, Props, Clone)]
pub struct InputTagsEvent {
    pub tags: Vec<String>,
}
#[derive(PartialEq, Props, Clone)]
pub struct InputTagsProps {
    message: String,
    placeholder: String,
    #[props(!optional)]
    error: Option<String>,
    #[props(default = ElementSize::Medium)]
    size: ElementSize,
    label: Option<String>,
    #[props(default = false)]
    required: bool,
    #[props(default = 5)]
    maxlength: u8,
    on_input: EventHandler<InputTagsEvent>,
    on_keypress: EventHandler<KeyboardEvent>,
    on_click: EventHandler<MouseEvent>,
}
pub fn InputTags(props: InputTagsProps) -> Element {
    let input_error_container = if props.error.is_some() {
        "input--error-container"
    } else {
        ""
    };
    let size = match props.size {
        ElementSize::Big => "input-wrapper__container--big",
        ElementSize::Medium => "input-wrapper__container--medium",
        ElementSize::Small => "input-wrapper__container--small",
    };

    let is_active = use_signal::<bool>(|| false);
    let mut tags = use_signal::<Vec<String>>(|| {
        if !props.message.is_empty() {
            props
                .message
                .split(',')
                .map(String::from)
                .collect::<Vec<String>>()
        } else {
            vec![]
        }
    });
    let mut complete_value = use_signal(String::new);
    let mut new_value = use_signal(String::new);
    let mut temporal_value = use_signal(String::new);
    let mut is_editing_tag = use_signal(|| None);
    rsx!(
        section {
            class: "input__wrapper",
            class: if is_active() { "input__wrapper--active" },
            if let Some(value) = props.label {
                label { class: "input__label", "{value}" }
            }
            div { class: "input-wrapper {size} {input_error_container}",
                {
                    tags().iter().enumerate().map(|(index, tag)| {
                        rsx!(
                            div {
                                class: "tag",
                                class: if let Some(i) = is_editing_tag() {
                                    // if i != index { "tag--editing" }
                                    // else {""}
                                    "tag--editing"
                                },
                                button {
                                    class: "tag__text",
                                    onclick: move |_| {
                                        if let Some(i) = is_editing_tag() {
                                            if i == index {
                                                new_value.set(temporal_value());
                                                is_editing_tag.set(None);
                                            } else {
                                                is_editing_tag.set(Some(index));
                                                new_value.set(tags()[index].clone());
                                            }
                                        } else {
                                            is_editing_tag.set(Some(index));
                                            temporal_value.set(new_value());
                                            new_value.set(tags()[index].clone());
                                        }
                                    },
                                    "{tag}"
                                }
                                IconButton {
                                    class: "button--drop bg--transparent",
                                    body: rsx!(
                                        Icon {
                                            icon: Close,
                                            height: 20,
                                            width: 20,
                                            fill: "var(--fill-400)"
                                        }
                                    ),
                                    on_click: move |_| {
                                        if let Some(i) = is_editing_tag() {
                                            if i == index {
                                                new_value.set(temporal_value());
                                                is_editing_tag.set(None);
                                            }
                                        }
                                        tags.with_mut(|t|t.remove(index));
                                        complete_value.set(tags().join(","));
                                    }
                                }
                            }
                        )
                    })
                },
                if !new_value().trim().is_empty() {
                    div {
                        class: "tag",
                        class: if is_editing_tag().is_some() { "tag--editing" },
                        button {
                            class: "tag__text",
                            onclick: move |_| {
                                new_value.set(temporal_value());
                                is_editing_tag.set(None);
                            },
                            if !temporal_value().is_empty() {
                                "{temporal_value()}"
                            } else {
                                "{new_value()}"
                            }
                        }
                        IconButton {
                            class: "button--drop bg--transparent",
                            body: rsx! {
                                Icon { icon: Close, height: 20, width: 20, fill: "var(--fill-400)" }
                            },
                            on_click: move |_| {
                                new_value.set(tags()[tags().len() - 1].clone());
                            }
                        }
                    }
                }
                input {
                    r#type: "text",
                    class: "input",
                    value: new_value,
                    required: props.required,
                    placeholder: if props.required { format!("{}*", props.placeholder) } else { props.placeholder },
                    oninput: move |event| {
                        if let Some(index) = is_editing_tag() {
                            tags.with_mut(|t| t[index] = event.value());
                            props
                                .on_input
                                .call(InputTagsEvent {
                                    tags: tags().clone(),
                                });
                            return;
                        }
                        if event.value().contains(',') {
                            if tags().len() == (props.maxlength - 1) as usize {
                                props
                                    .on_input
                                    .call(InputTagsEvent {
                                        tags: tags().clone(),
                                    });
                                return;
                            }
                            let e: Vec<String> = event
                                .value()
                                .split(',')
                                .map(|s| s.to_string())
                                .collect();
                            let last_tag = e[1].clone();
                            if !last_tag.is_empty() {
                                tags.with_mut(|t| t.push(e[0].clone()));
                                complete_value.set(tags().join(","));
                                new_value.set(last_tag.clone());
                            } else {
                                new_value.set(event.value().clone());
                            }
                        } else {
                            new_value.set(event.value().clone());
                        }
                        if event.value().is_empty() && tags.last().is_some() {
                            new_value.set(tags().last().unwrap().to_string());
                            tags.with_mut(|t| t.pop());
                        }
                        let val = if !temporal_value().is_empty() {
                            temporal_value()
                        } else {
                            new_value()
                        };
                        let mut t = tags().clone();
                        t.push(val.to_string());
                        props.on_input.call(InputTagsEvent { tags: t });
                    }
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
