use dioxus::prelude::*;
use super::icon::IconShape;
#[derive(PartialEq, Clone)]
pub struct ChevronRight;
impl IconShape for ChevronRight {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            path { d: "M14.5358 12L9.29285 6.72124C8.90238 6.32748 8.90238 5.68907 9.29285 5.29532C9.68331 4.90156 10.3164 4.90156 10.7068 5.29532L16.6567 11.2869C17.0473 11.6807 17.0473 12.3193 16.6567 12.7131L10.7068 18.7047C10.3164 19.0984 9.68331 19.0984 9.29285 18.7047C8.90238 18.3109 8.90238 17.6725 9.29285 17.2788L14.5358 12Z" }
        )
    }
}
