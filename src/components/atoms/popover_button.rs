use crate::components::atoms::{icon_button::Variant, IconButton};

use super::dropdown::ElementSize;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct PopoverProps {
    body: Element,
    #[props(default = "".to_string())]
    class: String,
    #[props(default = Variant::Round)]
    variant: Variant,
    #[props(default = ElementSize::Big)]
    size: ElementSize,
    text: String,
}
pub fn Popover(props: PopoverProps) -> Element {
    let mut show_over = use_signal(|| false);
    rsx!(
        div { class: "popover",
            if show_over() {
                div { class: "popover__tooltip",
                    span {class: "popover__message", { props.text }}
                }
            }
            IconButton {
                class: "button--drop {props.class}",
                variant: Variant::Round,
                size: ElementSize::Big,
                body: props.body,
                on_click: move |_| { show_over.toggle(); }
            }
        }
    )
}
