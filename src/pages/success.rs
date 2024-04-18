use dioxus::prelude::*;

use crate::{
    components::atoms::{Button, Subtitle, Title},
    pages::route::Route,
};

#[component]
pub fn Success() -> Element {
    let nav = use_navigator();
    rsx! {
        div {
            class: "success",
            Title {
                text: "¡Perfecto! \n¡Haz creado tu comunidad!🚀"
            }
            div {
                class: "success__subtitle",
                Subtitle {
                    text: "Virto te da la bienvenida a un mundo lleno de posibilidades para tu organización."
                }
            }
            div {
                class: "button--floating",
                Button {
                    text: "Continuar",
                    status: None,
                    on_click: move |_| {
                        nav.push(Route::Dash { });
                    },
                }
            }
        }
    }
}
