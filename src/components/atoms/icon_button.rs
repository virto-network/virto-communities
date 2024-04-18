use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct IconButtonProps {
    body: Element,
    #[props(default = "".to_string())]
    class: String,
    on_click: EventHandler<MouseEvent>,
}

pub fn IconButton(props: IconButtonProps) -> Element {
    rsx!(
        button {
            class: "button button--tertiary padding-reset {props.class}",
            onclick: move |event| props.on_click.call(event),
            {props.body}
        }
    )
}
