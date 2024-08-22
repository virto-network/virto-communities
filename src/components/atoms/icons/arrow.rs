use dioxus::prelude::*;
use super::icon::IconShape;
#[derive(PartialEq, Clone)]
pub struct Arrow;
impl IconShape for Arrow {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            path { d: "M16 10L12 14L8 10" }
        )
    }
}
