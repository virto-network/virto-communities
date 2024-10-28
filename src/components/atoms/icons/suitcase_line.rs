use dioxus::prelude::*;

use super::icon::IconShape;

#[derive(PartialEq, Clone, Debug)]
pub struct SuitcaseLine;
impl IconShape for SuitcaseLine {
    fn view_box(&self) -> String {
        String::from("0 0 56 56")
    }
    fn child_elements(&self) -> Element {
            rsx!(
            path {
                d: "M34.9998 7C36.2878 7 37.3332 8.04533 37.3332 9.33333V14H48.9998C50.2878 14 51.3332 15.0453 51.3332 16.3333V46.6667C51.3332 47.9547 50.2878 49 48.9998 49H6.99984C5.71184 49 4.6665 47.9547 4.6665 46.6667V16.3333C4.6665 15.0453 5.71184 14 6.99984 14H18.6665V9.33333C18.6665 8.04533 19.7118 7 20.9998 7H34.9998ZM37.3332 18.6667H18.6665V44.3333H37.3332V18.6667ZM9.33317 18.6667V44.3333H13.9998V18.6667H9.33317ZM32.6665 11.6667H23.3332V14H32.6665V11.6667ZM41.9998 18.6667V44.3333H46.6665V18.6667H41.9998Z"
            }
        )
    }
}
