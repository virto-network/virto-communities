use dioxus::prelude::*;
#[derive(PartialEq, Props, Clone)]
pub struct SwitchProps {
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
pub fn Switch(props: SwitchProps) -> Element {
    rsx!(
        label {
            class: "switch {props.class}",
            class: if props.checked { "switch--active" },
            input {
                r#type: "checkbox",
                name: props.name,
                disabled: props.disabled,
                checked: props.checked,
                onchange: move |_| { props.on_change.call(()) }
            }
            span { class: "slider"}
            span { class: "switch__label", {props.label} }
        }
    )
}
