use dioxus::prelude::*;
use super::icon::IconShape;
#[derive(PartialEq, Clone)]
pub struct Hamburguer;
impl IconShape for Hamburguer {
    fn view_box(&self) -> String {
        String::from("0 0 28 28")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            path { d: "M3.5 4.66699H24.5V7.00033H3.5V4.66699ZM3.5 12.8337H24.5V15.167H3.5V12.8337ZM3.5 21.0003H24.5V23.3337H3.5V21.0003Z" }
        )
    }
}
