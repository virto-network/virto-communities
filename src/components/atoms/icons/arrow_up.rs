use dioxus::prelude::*;

use super::icon::IconShape;

#[derive(PartialEq, Clone)]
pub struct ArrowUp;
impl IconShape for ArrowUp {
    fn view_box(&self) -> String {
        String::from("0 0 15 14")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            path { d: "M7.54297 2.63721V12.2353" }
            path { d: "M11.0332 5.25492L9.51731 3.73907C8.82625 3.04798 8.48066 2.70243 8.08225 2.57296C7.73172 2.45908 7.35419 2.45908 7.00366 2.57296C6.60525 2.70243 6.25968 3.04798 5.56859 3.73907L4.05273 5.25492" }
        )
    }
}
