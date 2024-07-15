use dioxus::prelude::*;

use super::icon::IconShape;

#[derive(PartialEq, Clone)]
pub struct SubstractLine;
impl IconShape for SubstractLine {
    fn view_box(&self) -> String {
        String::from("0 0 20 20")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            path { d: "M4.16675 9.16666H15.8334V10.8333H4.16675V9.16666Z" }
        )
    }
}
