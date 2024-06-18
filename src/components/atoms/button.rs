use dioxus::prelude::*;

use super::dropdown::ElementSize;

#[derive(PartialEq, Clone)]
pub enum Variant {
    Primary,
    Secondary,
    Tertiary,
}

#[derive(PartialEq, Props, Clone)]
pub struct ButtonProps {
    #[props(default = "".to_string())]
    class: String,
    text: String,
    #[props(default = Variant::Primary)]
    variant: Variant,
    #[props(default = ElementSize::Medium)]
    size: ElementSize,
    #[props(default = false)]
    disabled: bool,
    on_click: EventHandler<MouseEvent>,
    #[props(!optional)]
    status: Option<String>,
    left_icon: Option<Element>,
    right_icon: Option<Element>,
}

pub fn Button(props: ButtonProps) -> Element {
    let variant = match props.variant {
        Variant::Primary => "button--primary",
        Variant::Secondary => "button--secondary",
        Variant::Tertiary => "button--tertiary",
    };

    let size = match props.size {
        ElementSize::Big => "button--big",
        ElementSize::Medium => "button--medium",
        ElementSize::Small => "button--small",
    };

    let disabled = if props.disabled {
        "button--disabled"
    } else {
        ""
    };

    let loading = if props.status.is_some() {
        "button--loading"
    } else {
        ""
    };

    match &props.status {
        Some(s) => {
            rsx!( button { class: "button {props.class} {variant} {size} {loading}", disabled: true, "{s}" } )
        }
        None => {
            rsx!(
                button {
                    class: "button {props.class} {variant} {size} {disabled}",
                    disabled: props.disabled,
                    onclick: move |event| props.on_click.call(event),
                    {props.left_icon}
                    "{props.text}"
                    {props.right_icon}
                }
            )
        }
    }
}
