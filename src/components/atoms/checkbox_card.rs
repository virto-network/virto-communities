use dioxus::prelude::*;
#[derive(PartialEq, Props, Clone)]
pub struct CheckboxCardProps {
    #[props(default = "".to_string())]
    class: String,
    id: String,
    name: String,
    checked: bool,
    #[props(default = false)]
    soon: bool,
    body: Element,
    editable: Option<Element>,
    on_change: EventHandler,
}
pub fn CheckboxCard(props: CheckboxCardProps) -> Element {
    rsx!(
        label {
            class: "checkbox-card {props.class}",
            class: if props.soon { "checkbox-card--comming-soon" },
            class: if props.checked { "checkbox-card--active" },
            input {
                class: "checkbox__cta",
                r#type: "radio",
                name: props.name,
                disabled: props.soon,
                checked: props.checked,
                onchange: move |_| {
                    props.on_change.call(())
                }
            }
            div { class: "checkbox-card__header",
                div { class: "checkbox-custom" }
                {props.body}
            }
            if props.checked {
                if let Some(editable) = props.editable {
                    {editable}
                }
            }
            div { class: "checkbox-custom" }
        }
    )
}
