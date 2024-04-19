use dioxus::prelude::*;

use crate::{
    components::atoms::{avatar::Variant, AddPlus, Avatar, Hamburguer, Icon, IconButton},
    pages::route::Route,
};

#[component]
pub fn Sidebar() -> Element {
    let nav = use_navigator();
    let mut is_active = use_signal(|| false);

    let active_class = if is_active() { "sidebar--active" } else { "" };

    rsx!(
       section {
           class: "sidebar {active_class}",
           IconButton {
                class: "button--hamburguer",
                body: rsx!(
                    Icon {
                        icon: Hamburguer,
                        height: 30,
                        width: 30,
                        stroke_width: 2,
                        stroke: "var(--text-1)"
                    }
                ),
                on_click: move |_| {
                    is_active.toggle();
                }
            }
           div { class: "sidebar__container",
                div {
                    class: "sidebar__scream",
                    onclick: move |_| {
                        is_active.toggle();
                    }
                }
                ul { class: "sidebar__list",
                    li { class: "sidebar__item",
                        onclick: move |_|{},
                        IconButton {
                            class: "button--avatar",
                            body: rsx!(
                                Avatar {
                                    name: "V",
                                    size: 60,
                                    uri: None,
                                    variant: Variant::SemiRound
                                }
                            ),
                            on_click: move |_| { }
                        }
                        span {
                            "virto"
                        },
                    }
                    li { class: "sidebar__item",
                        onclick: move |_|{},
                        IconButton {
                            class: "button--icon",
                            body: rsx!(
                                Icon {
                                    icon: AddPlus,
                                    height: 60,
                                    width: 60,
                                    stroke_width: 1,
                                    stroke: "var(--text-white)"
                                }
                            ),
                            on_click: move |_| {
                                nav.push(Route::Discover {});
                            }
                        }
                        span {
                            "Agregar comunidad"
                        }
                    }
                }
            }
       }
    )
}
