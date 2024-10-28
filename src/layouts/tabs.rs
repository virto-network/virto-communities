use crate::{components::molecules::Tab, pages::route::Route};
use dioxus::prelude::*;
#[component]
pub fn Tabs() -> Element {
    rsx! {
        div { class: "page",
            div { class: "layout layout--authenticated grid-header", Tab {} }
            Outlet::<Route> {}
        }
    }
}
