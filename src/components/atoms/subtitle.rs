use dioxus::prelude::*;
#[derive(PartialEq, Props, Clone)]
pub struct ButtonProps {
    text: String,
}
pub fn Subtitle(props: ButtonProps) -> Element {
    rsx!(
        h3 { class: "subtitle", "{props.text}" }
    )
}
