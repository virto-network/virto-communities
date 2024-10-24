use dioxus::prelude::*;

use super::icon::IconShape;

#[derive(PartialEq, Clone)]
pub struct ArrowRightUpLine;
impl IconShape for ArrowRightUpLine {
    fn view_box(&self) -> String {
        String::from("0 0 25 24")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            path {
                d: "M16.5044 9.414L7.8974 18.021L6.4834 16.607L15.0894 8H7.5044V6H18.5044V17H16.5044V9.414Z",
                fill: "#56C960"
            }
        )
    }
}
