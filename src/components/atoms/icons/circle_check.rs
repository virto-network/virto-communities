use dioxus::prelude::*;
use super::icon::IconShape;
#[derive(PartialEq, Clone)]
pub struct CircleCheck;
impl IconShape for CircleCheck {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            path {
                d: "M15 10L11 14L9 12M12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12C21 16.9706 16.9706 21 12 21Z",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round"
            }
        )
    }
}
