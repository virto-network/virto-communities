use dioxus::prelude::*;
use super::icon::IconShape;
#[derive(PartialEq, Clone)]
pub struct ChevronLeft;
impl IconShape for ChevronLeft {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            path { d: "M9.41379 12L14.6568 17.2788C15.0473 17.6725 15.0473 18.3109 14.6568 18.7047C14.2663 19.0984 13.6333 19.0984 13.2428 18.7047L7.29289 12.7131C6.90237 12.3192 6.90237 11.6807 7.29289 11.2869L13.2428 5.29532C13.6333 4.90156 14.2663 4.90156 14.6568 5.29532C15.0473 5.68907 15.0473 6.32748 14.6568 6.72124L9.41379 12Z" }
        )
    }
}
