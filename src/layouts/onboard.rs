use dioxus::prelude::*;
use crate::{components::molecules::Sidebar, pages::route::Route};
#[component]
pub fn Onboard() -> Element {
    rsx! {
        Outlet::<Route> {}
    }
}
