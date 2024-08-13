use dioxus::prelude::*;

use super::dropdown::ElementSize;

#[derive(PartialEq, Props, Clone)]
pub struct RequestProps {
    name: String,
    details: Option<String>,
    #[props(default = ElementSize::Medium)]
    size: ElementSize,
}

pub fn Request(props: RequestProps) -> Element {
    let size = match props.size {
        ElementSize::Big => "vote-card__request--big",
        ElementSize::Medium => "vote-card__request--medium",
        ElementSize::Small => "vote-card__request--small",
    };

    rsx!(
        div {
            class: "vote-card__request {size}",
            span { class: "vote-card__request__title",
                {props.name}
            }
            if let Some(details) = props.details {
                span { class: "vote-card__request__details",
                    {details}
                }
            }
        }
    )
}
