use dioxus::prelude::*;
use super::icon::IconShape;
#[derive(PartialEq, Clone)]
pub struct OnOff;
impl IconShape for OnOff {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            path { d: "M8.5 4c-2 1-3.39 2.59-3.99 4.192a8.001 8.001 0 0 0 12.107 9.341 8 8 0 0 0 2.985-9.025C19.07 6.882 17.5 5 16 4m-3.83 8V3.843" }
        )
    }
}
