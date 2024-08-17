use dioxus::prelude::*;

use super::dropdown::ElementSize;

#[derive(PartialEq, Props, Clone)]
pub struct RequestProps {
    name: String,
    details: Option<String>,
    #[props(default = ElementSize::Medium)]
    size: ElementSize,
}

pub fn ActionRequest(props: RequestProps) -> Element {
    let size = match props.size {
        ElementSize::Big => "action-request--big",
        ElementSize::Medium => "action-request--medium",
        ElementSize::Small => "action-request--small",
    };

    rsx!(
        div {
            class: "action-request {size}",
            span { class: "action-request__title",
                {props.name}
            }
            if let Some(details) = props.details {
                span { class: "action-request__details",
                    {details}
                }
            }
        }
    )
}
