use dioxus::prelude::*;

use crate::{
    components::atoms::{Button, MessageInput, Title},
    hooks::{
        use_communities::{use_communities, Community},
        use_onboard::use_onboard,
    },
    pages::route::Route,
};

#[component]
pub fn Home() -> Element {
    let nav = use_navigator();
    let mut onboard = use_onboard();
    let mut communities = use_communities();

    rsx! {
        div {
            class: "home",
            Title {
                text: "Hola, 👋"
            }
            section {
                class: "home__form",
                MessageInput {
                    message: "{onboard.get().username}",
                    label: translate!(i18, "home.form.username.label"),
                    placeholder: translate!(i18, "home.form.username.placeholder"),
                    error: None,
                    on_input: move |event: FormEvent| {
                        onboard.set_username(event.value());
                    },
                    on_keypress: move |_| {},
                    on_click: move |_| {},
                }
            }
            div {
                class: "button--floating",
                Button {
                    text: "Continuar",
                    status: None,
                    on_click: move |_| {
                        nav.push(Route::Discover { });
                    },
                }
            }
        }
    }
}