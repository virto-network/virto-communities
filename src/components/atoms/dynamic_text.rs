use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Props, PartialEq)]
pub struct DynamicTextProps {
    pub words: Vec<String>,
}

pub fn DynamicText(props: DynamicTextProps) -> Element {
    rsx! {
        div { class: "animated-text",
           {
            props.words.iter().enumerate().map(|(index, word)| {
                rsx! {
                    span { key: "{index}", "{word}" }
                }
            })
           }
        },
    }
}