use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct CheckboxCardProps {
    id: String,
    text: String,
    emoji: String,
    on_change: EventHandler,
}

pub fn CheckboxCard(props: CheckboxCardProps) -> Element {
    rsx!(
        label {
            class: "checkbox-card",
            input {
                class: "checkbox__cta",
                r#type: "checkbox",
                onchange: move |event| {
                    log::info!("checkbox changed: {:?}", event);
                }
            }
            div {
                class: "checkbox-card__media",
                span {
                    class: "checkbox-card__emoji",
                    "{props.emoji}"
                }
            }
            span {
                class: "checkbox-card__title",
                "{props.text}"
            }
        }
    )
}
