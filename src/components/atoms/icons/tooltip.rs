use dioxus::prelude::*;

use super::icon::IconShape;

#[derive(PartialEq, Clone)]
pub struct TooltipIcon;
impl IconShape for TooltipIcon {
    fn view_box(&self) -> String {
        String::from("0 0 195 51")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            g {
                filter: "url(#a)",
                mask {
                    id: "b",
                    fill: "#fff",
                    path {
                        fill_rule: "evenodd",
                        clip_rule: "evenodd",
                        d: "M11 3a8 8 0 0 0-8 8v23a8 8 0 0 0 8 8h81.638l5.304 5 5.304-5H184a8 8 0 0 0 8-8V11a8 8 0 0 0-8-8z",
                    }
                },
                path {
                    fill_rule: "evenodd",
                    clip_rule: "evenodd",
                    d: "M11 3a8 8 0 0 0-8 8v23a8 8 0 0 0 8 8h81.638l5.304 5 5.304-5H184a8 8 0 0 0 8-8V11a8 8 0 0 0-8-8z",
                    fill: "#F0FDF1",
                },
                path {
                    d: "m92.638 42 .686-.728-.29-.272h-.396zm5.304 5-.686.728.686.646.686-.646zm5.304-5v-1h-.397l-.289.272zM4 11a7 7 0 0 1 7-7V2a9 9 0 0 0-9 9zm0 23V11H2v23zm7 7a7 7 0 0 1-7-7H2a9 9 0 0 0 9 9zm81.638 0H11v2h81.638zm5.99 5.272-5.304-5-1.372 1.456 5.304 5zm3.932-5-5.304 5 1.372 1.456 5.304-5zM184 41h-80.754v2H184zm7-7a7 7 0 0 1-7 7v2a9 9 0 0 0 9-9zm0-23v23h2V11zm-7-7a7 7 0 0 1 7 7h2a9 9 0 0 0-9-9zM11 4h173V2H11z",
                    fill: "#89EC96",
                    mask: "url(#b)",
                }
            },
        )
    }
}
