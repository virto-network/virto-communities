use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct ButtonProps {
    text: String,
}

pub fn Title(props: ButtonProps) -> Element {
    rsx!( h2 { class: "title", "{props.text}" } )
}
