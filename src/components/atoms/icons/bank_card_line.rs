use dioxus::prelude::*;

use super::icon::IconShape;

#[derive(PartialEq, Clone)]
pub struct BankCardLine;
impl IconShape for BankCardLine {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            path { d: "M3 3H21C21.2652 3 21.5196 3.10536 21.7071 3.29289C21.8946 3.48043 22 3.73478 22 4V20C22 20.2652 21.8946 20.5196 21.7071 20.7071C21.5196 20.8946 21.2652 21 21 21H3C2.73478 21 2.48043 20.8946 2.29289 20.7071C2.10536 20.5196 2 20.2652 2 20V4C2 3.73478 2.10536 3.48043 2.29289 3.29289C2.48043 3.10536 2.73478 3 3 3ZM20 11H4V19H20V11ZM20 9V5H4V9H20ZM14 15H18V17H14V15Z" }
        )
    }
}