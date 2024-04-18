use dioxus::prelude::*;

use crate::{
    components::atoms::{Button, Title},
    pages::route::Route,
};

#[component]
pub fn Dash() -> Element {
    let nav = use_navigator();
    rsx! {
        div {
            class: "dash",
            Title {
                text: "¿Qué quieres hacer ahora?"
            }
            div { class: "dash__form",
                Button {
                    text: "Agregar miembro 💁",
                    status: None,
                    on_click: move |_| {
                        nav.push(Route::Member { });
                    },
                }
                Button {
                    text: "Ir al chat 👋",
                    status: None,
                    on_click: move |_| {
                        nav.push(Route::Dash { });
                    },
                }
            }
        }
    }
}
