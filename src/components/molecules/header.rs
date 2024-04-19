use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

use crate::components::atoms::{Button, Subtitle};

#[component]
pub fn Header() -> Element {
    let i18 = use_i18();

    rsx!(
       header {
           class: "header",
           Subtitle {
                text: translate!(i18, "header.title")
           }
           div { class: "header__memberships",
                button { class: "button button--primary",
                    "🚀 "
                    span {
                        {translate!(i18, "header.options.memberships")}
                    }
                    "10"
                }
           }
       }
    )
}
