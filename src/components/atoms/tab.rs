use dioxus::prelude::*;
use super::dropdown::ElementSize;
#[derive(PartialEq, Props, Clone)]
pub struct TabProps {
    #[props(default = "".to_string())]
    class: String,
    text: String,
    #[props(default = false)]
    is_active: bool,
    #[props(default = ElementSize::Medium)]
    size: ElementSize,
    on_click: EventHandler<MouseEvent>,
    left_icon: Option<Element>,
    right_icon: Option<Element>,
}
pub fn Tab(props: TabProps) -> Element {
    let size = match props.size {
        ElementSize::Big => "tab--big",
        ElementSize::Medium => "tab--medium",
        ElementSize::Small => "tab--small",
    };
    rsx!(
        button {
            class: "tab {size} {props.class}",
            class: if props.is_active { "tab--active" },
            onclick: move |event| props.on_click.call(event),
            {props.left_icon},
            "{props.text}"
            { props.right_icon }
        }
    )
}
