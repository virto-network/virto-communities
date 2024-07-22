use dioxus::prelude::*;

use super::icon::IconShape;

#[derive(PartialEq, Clone)]
pub struct CircleShape;
impl IconShape for CircleShape {
    fn view_box(&self) -> String {
        String::from("0 0 16 16")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            circle { cx: "8", cy: "8", r: "7" }
        )
    }
}
