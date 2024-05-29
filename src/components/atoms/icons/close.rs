use dioxus::prelude::*;

use super::icon::IconShape;

#[derive(PartialEq, Clone)]
pub struct Close;
impl IconShape for Close {
    fn view_box(&self) -> String {
        String::from("0 0 32 32")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            path { d: "M12.0002 10.586L16.9502 5.63599L18.3642 7.04999L13.4142 12L18.3642 16.95L16.9502 18.364L12.0002 13.414L7.05023 18.364L5.63623 16.95L10.5862 12L5.63623 7.04999L7.05023 5.63599L12.0002 10.586Z" }
        )
    }
}
