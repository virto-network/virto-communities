use dioxus::prelude::*;
#[derive(PartialEq, Props, Clone)]
pub struct RadioButtonProps {
    #[props(default = "".to_string())]
    class: String,
    name: String,
    title: String,
    checked: bool,
    on_change: EventHandler,
}
pub fn RadioButton(props: RadioButtonProps) -> Element {
    rsx!(
        label { class: "radio-button {props.class}",
            input {
                class: "ration-button__cta",
                r#type: "radio",
                checked: props.checked,
                onchange: move |_| { props.on_change.call(()) }
            }
            div { class: "radio-button__header",
                div { class: "radio-custom" }
                span { {props.title} }
            }
        }
    )
}
