use dioxus::prelude::*;
use crate::{
    components::molecules::{Header, Sidebar},
    pages::route::Route,
};
#[component]
pub fn Authenticated() -> Element {
    rsx! {
        div { class: "page",
            div { class: "layout layout--onboard grid-sidebar", Sidebar {} }
            div { class: "layout layout--authenticated grid-header", Header {} }
            Outlet::<Route> {}
        }
    }
}
