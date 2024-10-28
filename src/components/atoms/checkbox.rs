use dioxus::prelude::*;
#[derive(PartialEq, Props, Clone)]
pub struct CheckboxProps {
    #[props(default = "".to_string())]
    class: String,
    id: String,
    name: String,
    checked: bool,
    label: String,
    #[props(default = false)]
    disabled: bool,
    on_change: EventHandler,
}
pub fn Checkbox(props: CheckboxProps) -> Element {
    rsx!(
        label {
            class: "container {props.class}",
            input {
                r#type: "checkbox",
                name: props.name,
                checked: props.checked,
                onchange: move |_| { props.on_change.call(()) }
            }
            span {class: "checkmark"}
            span {class: "checkbox__label", {props.label} }
        }
    )
}
