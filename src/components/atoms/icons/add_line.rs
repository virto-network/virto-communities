use dioxus::prelude::*;

use super::icon::IconShape;

#[derive(PartialEq, Clone)]
pub struct AddLine;
impl IconShape for AddLine {
    fn view_box(&self) -> String {
        String::from("0 0 20 20")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            path {
                d: "M9.16699 9.16797V4.16797H10.8337V9.16797H15.8337V10.8346H10.8337V15.8346H9.16699V10.8346H4.16699V9.16797H9.16699Z",
                fill: "#F0FDF1"
            }
        )
    }
}
