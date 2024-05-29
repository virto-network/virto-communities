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
            path { d: "M3.5 4.66666H24.5V6.99999H3.5V4.66666ZM3.5 12.8333H24.5V15.1667H3.5V12.8333ZM3.5 21H24.5V23.3333H3.5V21Z" }
        )
    }
}
