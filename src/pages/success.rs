use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::atoms::{Button, Subtitle, Title},
    pages::route::Route,
};

#[component]
pub fn Success() -> Element {
    let i18 = use_i18();
    let nav = use_navigator();

    rsx! {
        div {
            class: "success",
            Title {
                text: translate!(i18, "success.title")
            }
            div {
                class: "success__subtitle",
                Subtitle {
                    text: translate!(i18, "success.subtitle")
                }
            }
            div {
                class: "button--floating",
                Button {
                    text: translate!(i18, "success.form.cta_send"),
                    status: None,
                    on_click: move |_| {
                        nav.push(Route::Dash { });
                    },
                }
            }
        }
    }
}
