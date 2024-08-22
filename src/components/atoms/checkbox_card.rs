use dioxus::prelude::*;
#[derive(PartialEq, Props, Clone)]
pub struct CheckboxCardProps {
    id: String,
    title: String,
    description: String,
    icon: Element,
    name: String,
    checked: bool,
    #[props(default = false)]
    soon: bool,
    on_change: EventHandler,
}
pub fn CheckboxCard(props: CheckboxCardProps) -> Element {
    rsx!(
        label {
            class: "checkbox-card",
            class: if props.soon { "checkbox-card--comming-soon" },
            div { class: "checkbox-card__media",
                { props
                .icon }
            }
            div {
                span { class: "checkbox-card__title", "{props.title}" }
                p { class: "checkbox-card__description", "{props.description}" }
            }
            input {
                class: "checkbox__cta",
                r#type: "radio",
                name: props.name,
                disabled: props.soon,
                checked: props.checked,
                onchange: move |_| { props.on_change.call(()) }
            }
            div { class: "checkbox-custom" }
        }
    )
}
