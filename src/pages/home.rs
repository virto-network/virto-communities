use dioxus::prelude::*;

use crate::{
    components::atoms::{Button, MessageInput, Title},
    pages::route::Route,
};

#[component]
pub fn Home() -> Element {
    let nav = use_navigator();
    rsx! {
        div {
            class: "home",
            Title {
                text: "Hola, 👋"
            }
            section {
                class: "home__form",
                MessageInput {
                    message: "",
                    label: "Escribe tu nombre de usuario",
                    placeholder: "Ej: pepe",
                    error: None,
                    on_input: move |_| {},
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