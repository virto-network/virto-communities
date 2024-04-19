use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::atoms::{Button, MessageInput, Title},
    pages::route::Route,
};

#[component]
pub fn Member() -> Element {
    let i18 = use_i18();
    let nav = use_navigator();

    rsx! {
        div {
            class: "dash",
            Title {
                text: translate!(i18, "member.title")
            }
            div { class: "dash__form",
                MessageInput {
                    message: "",
                    label: translate!(i18, "member.form.address.label"),
                    placeholder: translate!(i18, "member.form.address.placeholder"),
                    error: None,
                    on_input: move |_| {},
                    on_keypress: move |_| {},
                    on_click: move |_| {},
                }
            }
            div {
                class: "button--floating",
                Button {
                    text: translate!(i18, "member.form.cta_send"),
                    status: None,
                    on_click: move |_| {
                        nav.push(Route::Discover { });
                    },
                }
            }
        }
    }
}
