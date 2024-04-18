use dioxus::prelude::*;

use crate::{
    components::atoms::{
        dropdown::DropdownItem, Attach, Button, Dropdown, MessageInput, TextareaInput, Title,
    },
    pages::route::Route,
};

#[component]
pub fn Discover() -> Element {
    let nav = use_navigator();
    let mut dropdown_value = use_signal(|| None);
    let dropdown_items = vec![
        DropdownItem {
            key: String::from("ecommerce"),
            value: String::from("Comercio"),
        },
        DropdownItem {
            key: String::from("Politician"),
            value: String::from("Político"),
        },
        DropdownItem {
            key: String::from("Gaming"),
            value: String::from("Video juegos"),
        },
        DropdownItem {
            key: String::from("educational"),
            value: String::from("Educación"),
        },
        DropdownItem {
            key: String::from("write"),
            value: String::from("Prefiero escribirlo"),
        },
    ];

    rsx! {
        div {
            class: "discover",
            Title {
                text: "Crea tu propia \ncomunidad 🙌"
            }
            div {
                class: "discover__form row",
                section {
                    class: "discover__form__wrapper",
                    MessageInput {
                        message: "",
                        label: "Escribe el nombre de tu comunidad",
                        placeholder: "Ej: PartyCalls",
                        error: None,
                        on_input: move |_| {},
                        on_keypress: move |_| {},
                        on_click: move |_| {},
                    }
                    Dropdown {
                        value: dropdown_value(),
                        items: dropdown_items,
                        label: "Industria",
                        default: None,
                        on_change: move |event: DropdownItem| {
                            dropdown_value.set(Some(event))
                        }
                    }
                    if let Some(v) = dropdown_value() {
                        if v.key == "write" {
                            MessageInput {
                                message: "",
                                label: "Escribe la industria",
                                placeholder: "Ej: marketing",
                                error: None,
                                on_input: move |_| {},
                                on_keypress: move |_| {},
                                on_click: move |_| {},
                            }
                        }
                    }
                    TextareaInput {
                        value: String::from(""),
                        placeholder: "Cuéntanos por qué tu comunidad es única y todos deberíann unirse",
                        label: "Descripción",
                        on_input: move |_| {},
                        on_keypress: move |_| {},
                        on_click: move |_| {},

                    }
                }
                div {
                    Attach {
                        // value: "",
                        label: "Personaliza tu comunidad con el logo",
                    }
                }
            }
            div {
                class: "button--floating",
                Button {
                    text: "Continuar",
                    status: None,
                    on_click: move |_| {
                        nav.push(Route::Custom { });
                    },
                }
            }
        }
    }
}
