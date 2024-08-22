use dioxus::prelude::*;
use super::icon::IconShape;
#[derive(PartialEq, Clone)]
pub struct Plus;
impl IconShape for Plus {
    fn view_box(&self) -> String {
        String::from("0 0 32 32")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            path { d: "M15.9993 6.66675V25.3334M6.66602 16.0001H25.3327" }
        )
    }
}
