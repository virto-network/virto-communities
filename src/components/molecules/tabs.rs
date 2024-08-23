use dioxus::prelude::*;
#[derive(PartialEq, Props, Clone, Default)]
pub struct TabItem {
    pub k: String,
    pub value: String,
}
#[derive(PartialEq, Props, Clone)]
pub struct TabsProps {
    body: Vec<Element>,
}
pub fn Tabs(props: TabsProps) -> Element {
    rsx!(
        section { class: "tabs",
            {props.body.into_iter().map(|item| {
                rsx!(
                    {item}
                )
            })}
        }
    )
}
