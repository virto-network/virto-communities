use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::atoms::{Button, MessageInput, Title},
    pages::route::Route,
};

#[component]
pub fn Home() -> Element {
    let i18 = use_i18();
    let nav = use_navigator();

    rsx! {
        div {
            class: "home",
            Title {
                text: translate!(i18, "home.title")
            }
            section {
                class: "home__form",
                MessageInput {
                    message: "",
                    label: translate!(i18, "home.form.username.label"),
                    placeholder: translate!(i18, "home.form.username.placeholder"),
                    error: None,
                    on_input: move |_| {},
                    on_keypress: move |_| {},
                    on_click: move |_| {},
                }
            }
            div {
                class: "button--floating",
                Button {
                    text: translate!(i18, "home.form.cta_send"),
                    status: None,
                    on_click: move |_| {
                        nav.push(Route::Discover { });
                    },
                }
            }
        }
    }
}
