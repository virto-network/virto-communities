use dioxus::prelude::*;
#[derive(PartialEq, Props, Clone)]
pub struct AutocompleteItemProps {
    title: String,
    // on_click: EventHandler<()>,
}
pub fn AutocompleteItemButton(props: AutocompleteItemProps) -> Element {
    rsx!(
        div { 
            class: "autocomplete__item--recipient",
            span { class: "autocomplete__item__alias", "{props.title}"}
        }
    )
}
