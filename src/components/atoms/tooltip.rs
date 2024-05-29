use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct TooltipProps {
    title: String,
    body: String,
}

pub fn Tooltip(props: TooltipProps) -> Element {
    rsx!(
        button {
            class: "tooltip",
            h3 { class: "tooltip__title", "{props.title}" }
            p { class: "tooltip__body", "{props.body}" }
        }
    )
}
