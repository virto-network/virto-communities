use dioxus::prelude::*;

use super::icon::IconShape;

#[derive(PartialEq, Clone)]
pub struct Settings;
impl IconShape for Settings {
    fn view_box(&self) -> String {
        String::from("0 0 32 32")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            path { d: "M16 1.33334L28.6667 8.66668V23.3333L16 30.6667L3.33334 23.3333V8.66668L16 1.33334ZM16 20C17.0609 20 18.0783 19.5786 18.8284 18.8284C19.5786 18.0783 20 17.0609 20 16C20 14.9391 19.5786 13.9217 18.8284 13.1716C18.0783 12.4214 17.0609 12 16 12C14.9391 12 13.9217 12.4214 13.1716 13.1716C12.4214 13.9217 12 14.9391 12 16C12 17.0609 12.4214 18.0783 13.1716 18.8284C13.9217 19.5786 14.9391 20 16 20V20Z" }
        )
    }
}
