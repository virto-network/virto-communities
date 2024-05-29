use dioxus::prelude::*;

use super::icon::IconShape;

#[derive(PartialEq, Clone)]
pub struct Chat;
impl IconShape for Chat {
    fn view_box(&self) -> String {
        String::from("0 0 32 32")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            path { d: "M8.60666 25.3333L2.66666 30V5.33333C2.66666 4.97971 2.80714 4.64057 3.05719 4.39052C3.30724 4.14048 3.64638 4 4 4H28C28.3536 4 28.6928 4.14048 28.9428 4.39052C29.1929 4.64057 29.3333 4.97971 29.3333 5.33333V24C29.3333 24.3536 29.1929 24.6928 28.9428 24.9428C28.6928 25.1929 28.3536 25.3333 28 25.3333H8.60666ZM9.33333 13.3333C9.33333 15.1014 10.0357 16.7971 11.286 18.0474C12.5362 19.2976 14.2319 20 16 20C17.7681 20 19.4638 19.2976 20.714 18.0474C21.9643 16.7971 22.6667 15.1014 22.6667 13.3333H20C20 14.3942 19.5786 15.4116 18.8284 16.1618C18.0783 16.9119 17.0609 17.3333 16 17.3333C14.9391 17.3333 13.9217 16.9119 13.1716 16.1618C12.4214 15.4116 12 14.3942 12 13.3333H9.33333Z" }
        )
    }
}
