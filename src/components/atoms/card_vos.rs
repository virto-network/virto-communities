use dioxus::prelude::*;
#[derive(PartialEq, Props, Clone)]
pub struct CardVosProps {
    title: String,
    description: String,
    #[props(default = false)]
    active: bool,
    icon: Element,
    on_click: EventHandler<MouseEvent>,
}
pub fn CardVos(props: CardVosProps) -> Element {
    rsx!(
        button {
            class: "card-vos",
            class: if !props.active { "card-vos--inactive" },
            onclick: move |event| props.on_click.call(event),
            div { class: "card-vos__head",
                h3 { class: "card-vos__title", "{props.title}" }
                {props.icon}
            }
            p { class: "card-vos__description", "{props.description}" }
        }
    )
}
