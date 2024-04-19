use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::atoms::{Button, Title},
    pages::route::Route,
};

#[component]
pub fn Dash() -> Element {
    let i18 = use_i18();
    let nav = use_navigator();

    rsx! {
        div {
            class: "dash",
            Title {
                text: translate!(i18, "dash.title")
            }
            div { class: "dash__form",
                Button {
                    text: translate!(i18, "dash.form.options.add_member"),
                    status: None,
                    on_click: move |_| {
                        nav.push(Route::Member { });
                    },
                }
                Button {
                    text: translate!(i18, "dash.form.options.chat"),
                    status: None,
                    on_click: move |_| {
                        nav.push(Route::Dash { });
                    },
                }
            }
        }
    }
}
