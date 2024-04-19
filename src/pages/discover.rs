use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::atoms::{
        dropdown::DropdownItem, Attach, Button, Dropdown, MessageInput, TextareaInput, Title,
    },
    pages::route::Route,
};

#[component]
pub fn Discover() -> Element {
    let i18 = use_i18();
    let nav = use_navigator();

    let mut dropdown_value = use_signal(|| None);
    let dropdown_items = vec![
        DropdownItem {
            key: String::from("commerce"),
            value: translate!(i18, "discover.form.industry.options.commerce"),
        },
        DropdownItem {
            key: String::from("olitical"),
            value: translate!(i18, "discover.form.industry.options.political"),
        },
        DropdownItem {
            key: String::from("gaming"),
            value: translate!(i18, "discover.form.industry.options.gaming"),
        },
        DropdownItem {
            key: String::from("educational"),
            value: translate!(i18, "discover.form.industry.options.education"),
        },
        DropdownItem {
            key: String::from("custom"),
            value: translate!(i18, "discover.form.industry.options.custom"),
        },
    ];

    rsx! {
        div {
            class: "discover",
            Title {
                text: translate!(i18, "discover.title")
            }
            div {
                class: "discover__form row",
                section {
                    class: "discover__form__wrapper",
                    MessageInput {
                        message: "",
                        label: translate!(i18, "discover.form.name.label"),
                        placeholder: translate!(i18, "discover.form.name.placeholder"),
                        error: None,
                        on_input: move |_| {},
                        on_keypress: move |_| {},
                        on_click: move |_| {},
                    }
                    Dropdown {
                        value: dropdown_value(),
                        items: dropdown_items,
                        label: translate!(i18, "discover.form.industry.label"),
                        default: None,
                        on_change: move |event: DropdownItem| {
                            dropdown_value.set(Some(event))
                        }
                    }
                    if let Some(v) = dropdown_value() {
                        if v.key == "custom" {
                            MessageInput {
                                message: "",
                                label: translate!(i18, "discover.form.custom_industry.label"),
                                placeholder: translate!(i18, "discover.form.custom_industry.placeholder"),
                                error: None,
                                on_input: move |_| {},
                                on_keypress: move |_| {},
                                on_click: move |_| {},
                            }
                        }
                    }
                    TextareaInput {
                        value: String::from(""),
                        placeholder: translate!(i18, "discover.form.description.placeholder"),
                        label: translate!(i18, "discover.form.description.label"),
                        on_input: move |_| {},
                        on_keypress: move |_| {},
                        on_click: move |_| {},

                    }
                }
                div {
                    Attach {
                        // value: "",
                        label: translate!(i18, "discover.form.logo.label"),
                        cta_text: translate!(i18, "discover.form.logo.placeholder")
                    }
                }
            }
            div {
                class: "button--floating",
                Button {
                    text: translate!(i18, "discover.form.cta_send"),
                    status: None,
                    on_click: move |_| {
                        nav.push(Route::Custom { });
                    },
                }
            }
        }
    }
}
