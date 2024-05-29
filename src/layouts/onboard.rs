use dioxus::prelude::*;

use crate::{components::molecules::Sidebar, pages::route::Route};

#[component]
pub fn Onboard() -> Element {
    rsx! {
        div {
            class: "layout layout--onboard grid-sidebar",
            Sidebar {}
        }
        Outlet::<Route> {}
    }
}
