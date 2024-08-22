use dioxus::prelude::*;
use super::dropdown::ElementSize;
#[derive(PartialEq, Clone)]
pub enum Variant {
    Primary,
    Secondary,
}
#[derive(PartialEq, Props, Clone)]
pub struct KeyValueProps {
    #[props(default = "".to_string())]
    class: String,
    #[props(default = ElementSize::Big)]
    size: ElementSize,
    #[props(default = Variant::Primary)]
    variant: Variant,
    text: String,
    body: Element,
}
pub fn KeyValue(props: KeyValueProps) -> Element {
    let size = match props.size {
        ElementSize::Big => "key-value--big",
        ElementSize::Medium => "key-value--medium",
        ElementSize::Small => "key-value--small",
    };
    let variant = match props.variant {
        Variant::Primary => "key-value--primary",
        Variant::Secondary => "key-value--secondary",
    };
    rsx!(
        span { class: "key-value {props.class} {size} {variant}",
            h4 { class: "key-value__key", "{props.text}" }
            div { class: "key-value__value",
                { props
                .body }
            }
        }
    )
}
