use dioxus::prelude::*;

use crate::pages::route::Route;

#[component]
pub fn Onboard() -> Element {
    rsx! {
        Outlet::<Route> {}
    }
}
