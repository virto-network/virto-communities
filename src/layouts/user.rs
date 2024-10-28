use dioxus::prelude::*;

use crate::{components::molecules::Header, pages::route::Route};

#[component]
pub fn User() -> Element {
    rsx! {
        div { class: "page",
            div { class: "layout layout--authenticated grid-header", Header {} }
            Outlet::<Route> {}
        }
    }
}
