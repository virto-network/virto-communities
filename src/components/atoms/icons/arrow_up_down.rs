use dioxus::prelude::*;
use super::icon::IconShape;
#[derive(PartialEq, Clone)]
pub struct ArrowUpDown;
impl IconShape for ArrowUpDown {
    fn view_box(&self) -> String {
        String::from("0 0 32 32")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            path { d: "M15.9333 10.6L14.048 12.4853L10.6667 9.104V26.6667H8V9.104L4.62 12.4853L2.73334 10.6L9.33334 4L15.9333 10.6ZM29.2667 21.4L22.6667 28L16.0667 21.4L17.952 19.5147L21.3347 22.896L21.3333 5.33333H24V22.896L27.3813 19.5147L29.2667 21.4V21.4Z" }
        )
    }
}
