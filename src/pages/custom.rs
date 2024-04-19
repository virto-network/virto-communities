use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::atoms::{Button, CheckboxCard, Subtitle, Title},
    pages::route::Route,
};

#[component]
pub fn Custom() -> Element {
    let i18 = use_i18();
    let nav = use_navigator();

    rsx! {
        div {
            class: "custom",
                Title {
                    text: translate!(i18, "custom.title")
                }
                div {
                    class: "custom__form",
                    Subtitle {
                        text: translate!(i18, "custom.subtitle")
                    }
                    div {
                        class: "custom__form__wrapper",
                        label { class: "input__label", {translate!(i18, "custom.form.solutions.label")} }
                        div {
                            class: "custom__checkbox__container",
                            CheckboxCard {
                                id: "token",
                                text: translate!(i18, "custom.form.solutions.options.token"),
                                emoji: "🪙",
                                on_change: move |_|{}
                            }
                            CheckboxCard {
                                id: "treasury",
                                text: translate!(i18, "custom.form.solutions.options.treasury"),
                                emoji: "🏛️",
                                on_change: move |_|{}
                            }
                            CheckboxCard {
                                id: "chat",
                                text: translate!(i18, "custom.form.solutions.options.chat"),
                                emoji: "👋",
                                on_change: move |_|{}
                            }
                        }
                    }
                }
                div {
                    class: "button--floating",
                    Button {
                        text: translate!(i18, "custom.form.cta_send"),
                        status: None,
                        on_click: move |_| {
                            nav.push(Route::Success {});
                        },
                    }
                }
        }
    }
}
