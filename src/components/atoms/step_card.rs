use dioxus::prelude::*;
#[derive(PartialEq, Props, Clone)]
pub struct StepCardProps {
    name: String,
    checked: bool,
    #[props(default = false)]
    soon: bool,
    #[props(default = "".to_string())]
    class: String,
    body: Element,
    editable: Option<Element>,
    on_change: EventHandler,
}
pub fn StepCard(props: StepCardProps) -> Element {
    rsx!(
        label {
            class: "step-card {props.class}",
            class: if props.soon { "step-card--comming-soon" },
            input {
                class: "step__cta",
                r#type: "radio",
                name: props.name,
                disabled: props.soon,
                checked: props.checked,
                onchange: move |_| { props.on_change.call(()) }
            }
            div { class: "step-card__header",
                div { class: "step-custom" }
                { props.body }
            }
            if props.checked {
                if let Some(editable) = props.editable {
                    { editable }
                }
            }
        }
    )
}
