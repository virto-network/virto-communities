use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum InputType {
    Text,
    Message,
    Search,
    Password,
}

#[derive(PartialEq, Props, Clone)]
pub struct MessageInputProps {
    #[props(default = InputType::Text)]
    itype: InputType,
    message: String,
    placeholder: String,
    #[props(!optional)]
    error: Option<String>,
    label: Option<String>,
    on_input: EventHandler<FormEvent>,
    on_keypress: EventHandler<KeyboardEvent>,
    on_click: EventHandler<MouseEvent>,
}

pub fn MessageInput(props: MessageInputProps) -> Element {
    let input_error_container = if let Some(_) = props.error {
        "input--error-container"
    } else {
        ""
    };

    let input_type = match props.itype {
        InputType::Text => "text",
        InputType::Search => "search",
        InputType::Message => "text",
        InputType::Password => "password",
    };

    rsx!(
        section {
            class: "input__wrapper",
            if let Some(value) = props.label {
                label { class: "input__label", "{value}" }
            }
            div {
                class: "input-wrapper {input_error_container}",
                input {
                    r#type: "{input_type}",
                    class: "input",
                    value: props.message,
                    placeholder: "{props.placeholder}",
                    oninput: move |event| props.on_input.call(event),
                    onkeypress: move |event| props.on_keypress.call(event)
                }
            }
            if let Some(error) = props.error {
                div {
                    class: "input--error",
                    "{error}"
                }
            }
        }
    )
}
