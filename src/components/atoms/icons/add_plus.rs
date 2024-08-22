use dioxus::prelude::*;
use super::icon::IconShape;
#[derive(PartialEq, Clone)]
pub struct AddPlus;
impl IconShape for AddPlus {
    fn view_box(&self) -> String {
        String::from("0 0 28 28")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            path { d: "M12.8333 12.8333V5.83334H15.1667V12.8333H22.1667V15.1667H15.1667V22.1667H12.8333V15.1667H5.83334V12.8333H12.8333Z" }
        )
    }
}
