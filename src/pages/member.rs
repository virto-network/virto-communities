use dioxus::prelude::*;

use crate::{
    components::atoms::{Button, MessageInput, Title},
    pages::route::Route,
};

#[component]
pub fn Member() -> Element {
    let nav = use_navigator();
    rsx! {
        div {
            class: "dash",
            Title {
                text: "Agregar miembro 💁"
            }
            div { class: "dash__form",
                MessageInput {
                    message: "",
                    label: "Escribe la dirección de billetera del nuevo miembro",
                    placeholder: "Ej: 5h1xa...",
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
