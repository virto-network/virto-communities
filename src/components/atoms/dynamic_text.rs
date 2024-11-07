use dioxus::prelude::*;

#[derive(Clone, Props, PartialEq)]
pub struct DynamicTextProps {
    pub words: Vec<String>,
}

pub fn DynamicText(props: DynamicTextProps) -> Element {
    rsx! {
        {
            {
                props.words.iter().enumerate().map(|(index, word)| {
                    rsx! {
                        span { key: "{index}", "{word}" }
                    }
                })
            }
        }
    }
}