use dioxus::prelude::*;

use super::icon::IconShape;

#[derive(PartialEq, Clone)]
pub struct WarningSign;
impl IconShape for WarningSign {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            path { fill_rule: "evenodd", clip_rule: "evenodd", d: "M14.2987 3.31824C13.2904 1.56059 10.7096 1.56059 9.7013 3.31824L2.34255 16.146C1.35361 17.8699 2.62409 20 4.64124 20H19.3588C21.3759 20 22.6464 17.8699 21.6575 16.146L14.2987 3.31824ZM12 6.5C11.4477 6.5 11 6.94772 11 7.5V13.5C11 14.0523 11.4477 14.5 12 14.5C12.5523 14.5 13 14.0523 13 13.5V7.5C13 6.94772 12.5523 6.5 12 6.5ZM12 15.5C11.4477 15.5 11 15.9477 11 16.5C11 17.0523 11.4477 17.5 12 17.5C12.5523 17.5 13 17.0523 13 16.5C13 15.9477 12.5523 15.5 12 15.5Z", fill: "#DC2828" }
        )
    }
}
