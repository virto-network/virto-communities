use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use uuid::Uuid;

use crate::{
    components::atoms::{
        dropdown::DropdownItem, Attach, Button, Dropdown, MessageInput, TextareaInput, Title,
    },
    hooks::{
        use_attach::use_attach,
        use_communities::{use_communities, Community},
    },
    pages::route::Route,
};

#[component]
pub fn Discover() -> Element {
    let nav = use_navigator();
    let attach = use_attach();
    let mut communities = use_communities();
    let mut community = use_signal::<Community>(|| Community::default());

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
                        message: "{community().name}",
                        label: translate!(i18, "discover.form.name.label"),
                        placeholder: translate!(i18, "discover.form.name.placeholder"),
                        error: None,
                        on_input: move |event: FormEvent| {
                            community.with_mut(|c| c.name = event.value() );
                        },
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
                        value: "{community().description}",
                        placeholder: translate!(i18, "discover.form.description.placeholder"),
                        label: translate!(i18, "discover.form.description.label"),
                        on_input: move |event: FormEvent| {
                            community.with_mut(|c| c.description = event.value() );
                        },
                        on_keypress: move |_| {},
                        on_click: move |_| {},

                    }
                }
                div {
                    Attach {
                        label: translate!(i18, "discover.form.logo.label"),
                        cta_text: translate!(i18, "discover.form.logo.placeholder")
                    }
                }
            }
            div {
                class: "button--floating",
                Button {
                    text: "Continuar",
                    status: None,
                    on_click: move |_| {
                        community.with_mut(|c|c.id = Uuid::new_v4().to_string());
                        
                        if let Some(item) = dropdown_value() {
                            community.with_mut(|c|c.industry = item)
                        };

                        if let Some(attachment) = attach.get() {
                            community.with_mut(|c|c.logo = Some(attachment.preview_url.to_string()))
                        }

                        communities.push(community());
                        nav.push(Route::Custom { });
                    },
                }
            }
        }
    }
}
