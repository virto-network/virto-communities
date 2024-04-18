use dioxus::prelude::*;

use crate::components::atoms::{Button, Subtitle};

#[component]
pub fn Header() -> Element {
    rsx!(
       header {
           class: "header",
           Subtitle {
                text: "Hola Virto"
           }
           div { class: "header__memberships",
                Button {
                    text: "🚀 Membresías: 10",
                    status: None,
                    on_click: move |_| {}
                }
           }
       }
    )
}
