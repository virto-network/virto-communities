use dioxus::prelude::*;
use crate::{
    components::molecules::Sidebar,
    pages::route::Route,
};
#[component]
pub fn Authenticated() -> Element {
    rsx! {
        div { class: "page",
            div { class: "layout layout--onboard grid-sidebar", Sidebar {} }
            Outlet::<Route> {}
        }
    }
}
