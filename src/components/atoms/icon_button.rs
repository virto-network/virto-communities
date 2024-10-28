use dioxus::prelude::*;
use super::dropdown::ElementSize;
#[derive(PartialEq, Clone)]
pub enum Variant {
    Round,
    SemiRound,
    Ghost,
    Secondary,
    Danger
}
#[derive(PartialEq, Props, Clone)]
pub struct IconButtonProps {
    body: Element,
    #[props(default = "".to_string())]
    class: String,
    #[props(default = Variant::Round)]
    variant: Variant,
    #[props(default = false)]
    disabled: bool,
    #[props(default = ElementSize::Big)]
    size: ElementSize,
    on_click: EventHandler<MouseEvent>,
}
pub fn IconButton(props: IconButtonProps) -> Element {
    let variant = match props.variant {
        Variant::Round => "icon-button--round",
        Variant::SemiRound => "icon-button--semi-round",
        Variant::Ghost => "icon-button--ghost",
        Variant::Secondary => "icon-button--secondary",
        Variant::Danger => "icon-button--danger",
    };
    let size = match props.size {
        ElementSize::Big => "icon-button--big",
        ElementSize::Medium => "icon-button--medium",
        ElementSize::Small => "icon-button--small",
    };
    rsx!(
        button {
            class: "button button--tertiary padding-reset {props.class} {variant} {size} commin-soon",
            class: if props.disabled { "button--disabled" },
            onclick: move |event| props.on_click.call(event),
            disabled: props.disabled,
            {props.body}
        }
    )
}
