use dioxus::prelude::*;

use super::icon::IconShape;

#[derive(PartialEq, Clone)]
pub struct MinusCircle;
impl IconShape for MinusCircle {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            path { fill_rule: "evenodd", clip_rule: "evenodd", d: "M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22ZM12 20C16.4183 20 20 16.4183 20 12C20 7.58172 16.4183 4 12 4C7.58172 4 4 7.58172 4 12C4 16.4183 7.58172 20 12 20Z" }
            path { d: "M16.5 11C17.0523 11 17.5 11.4477 17.5 12C17.5 12.5523 17.0523 13 16.5 13H7.5C6.94772 13 6.5 12.5523 6.5 12C6.5 11.4477 6.94772 11 7.5 11H16.5Z" }
        )
    }
}
