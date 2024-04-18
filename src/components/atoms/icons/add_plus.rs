use dioxus::prelude::*;

use super::icon::IconShape;

#[derive(PartialEq, Clone)]
pub struct AddPlus;
impl IconShape for AddPlus {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            path { d: "M6 12H12M12 12H18M12 12V18M12 12V6" }
        )
    }
}
