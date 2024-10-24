use dioxus::prelude::*;

use super::icon::IconShape;

#[derive(PartialEq, Clone)]
pub struct CheckboxCircleFill;
impl IconShape for CheckboxCircleFill {
    fn view_box(&self) -> String {
        String::from("0 0 32 33")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            path {
                d: "M15.9993 29.5915C8.63535 29.5915 2.66602 23.6221 2.66602 16.2581C2.66602 8.89414 8.63535 2.9248 15.9993 2.9248C23.3633 2.9248 29.3327 8.89414 29.3327 16.2581C29.3327 23.6221 23.3633 29.5915 15.9993 29.5915ZM14.67 21.5915L24.0967 12.1635L22.2113 10.2781L14.67 17.8208L10.898 14.0488L9.01268 15.9341L14.67 21.5915Z"
            }
        )
    }
}
