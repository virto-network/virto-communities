use dioxus::prelude::*;
#[derive(PartialEq, Props, Clone)]
pub struct TextareaInputProps {
    value: String,
    placeholder: String,
    label: Option<String>,
    help: Option<String>,
    on_input: EventHandler<FormEvent>,
    on_keypress: EventHandler<KeyboardEvent>,
    on_click: EventHandler<MouseEvent>,
}
pub fn TextareaInput(props: TextareaInputProps) -> Element {
    let mut sent_handled = use_signal(|| false);
    rsx!(
        section { class: "textarea",
            if let Some(value) = props.label {
                label { class: "input__label", "{value}" }
            }
            textarea {
                id: "textarea",
                class: "textarea__wrapper input",
                value: props.value.clone(),
                placeholder: "{props.placeholder}",
                oninput: move |event| {
                    props.on_input.call(event);
                },
                onkeypress: move |event| {
                    let modifiers = event.modifiers();
                    match modifiers {
                        keyboard_types::Modifiers::SHIFT => {}
                        _ => {
                            if event.code() == keyboard_types::Code::Enter {
                                sent_handled.set(true)
                            }
                        }
                    }
                    if modifiers.is_empty() {
                        props.on_keypress.call(event)
                    }
                }
            }
            if let Some(help) = props.help {
                div { class: "input--help", "{help}" }
            }
        }
    )
}
