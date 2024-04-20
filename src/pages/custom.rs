use dioxus::prelude::*;

use crate::{
    components::atoms::{Button, CheckboxCard, Subtitle, Title},
    hooks::use_communities::use_communities,
    pages::route::Route,
};

#[component]
pub fn Custom() -> Element {
    let nav = use_navigator();
    let communities = use_communities();
    
    rsx! {
        div {
            class: "custom",
                Title {
                    text: "Configura tu DAO, \nVirto 🚧️"
                }
                div {
                    class: "custom__form",
                    Subtitle {
                        text: "Virto te ofrece gran posibilidad de soluciones"
                    }
                    div {
                        class: "custom__form__wrapper",
                        label { class: "input__label", "Selecciona las que son de tu mayor interés y potencia tu organización." }
                        div {
                            class: "custom__checkbox__container",
                            CheckboxCard {
                                id: "a",
                                text: "Token",
                                emoji: "🪙",
                                on_change: move |_|{}
                            }
                            CheckboxCard {
                                id: "a",
                                text: "Tesorería",
                                emoji: "🏛️",
                                on_change: move |_|{}
                            }
                            CheckboxCard {
                                id: "a",
                                text: "Chat",
                                emoji: "👋",
                                on_change: move |_|{}
                            }
                        }
                    }
                }
                div {
                    class: "button--floating",
                    Button {
                        text: "Finalizar",
                        status: None,
                        on_click: move |_| {
                            nav.push(Route::Success {});
                        },
                    }
                }
        }
    }
}
