use dioxus::prelude::*;

use crate::{components::molecules::Header, pages::route::Route};

#[component]
pub fn Authenticated() -> Element {
    rsx! {
        div { class: "page",
            div { class: "layout layout--authenticated grid-header",
                Header {}
            }
            Outlet::<Route> {}
        }
    }
}
