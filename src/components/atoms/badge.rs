use dioxus::prelude::*;
#[derive(PartialEq, Props, Clone)]
pub struct BadgeProps {
    #[props(default = "".to_string())]
    class: String,
    text: String,
}
pub fn Badge(props: BadgeProps) -> Element {
    rsx!(
        span { class: "badge {props.class}", { props.text } }
    )
}
