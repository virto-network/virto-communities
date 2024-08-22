use dioxus::prelude::*;
#[derive(PartialEq, Clone)]
pub enum Variant {
    Remaign,
    Vote,
    Participation,
}
#[derive(PartialEq, Props, Clone)]
pub struct BarProps {
    left_value: f64,
    center_value: Option<f64>,
    right_value: f64,
    left_helper: Option<String>,
    right_helper: Option<String>,
    left_title: Option<String>,
    right_title: Option<String>,
    #[props(default = Variant::Remaign)]
    variant: Variant,
}
pub fn Bar(props: BarProps) -> Element {
    let variant = match props.variant {
        Variant::Remaign => "bar--remaign",
        Variant::Vote => "bar--vote",
        Variant::Participation => "bar--participation",
    };
    rsx!(
        section {
            div { class: "bar {variant}",
                span { class: "bar__content bar__content--left", style: format!("width: {}%", props.left_value),
                    p { class: "votes-counter__title", { props.left_helper } }
                }
                if let Some(value) = props.center_value {
                    span { class: "bar__content__threshold", style: format!("left: {}%", value) }
                }
                span { class: "bar__content bar__content--right", style: format!("width: {}%", props.right_value),
                    p { class: "votes-counter__title", { props.right_helper } }
                }
            }
            div { class: "bar__percent",
                p { class: "votes-counter__percent",
                    { props
                    .left_title }
                }
                p { class: "votes-counter__percent", { props.right_title } }
            }
        }
    )
}
