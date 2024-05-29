use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::atoms::{
        avatar::Variant,
        dropdown::ElementSize,
        icon_button::{self},
        AddPlus, Avatar, Compass, Hamburguer, Icon, IconButton,
    },
    hooks::use_communities::use_communities,
};

#[component]
pub fn Sidebar() -> Element {
    let i18 = use_i18();
    let communities = use_communities();

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
                    if !communities.get_community().name.is_empty() || communities.get_community().logo.is_some() {
                        li { class: "sidebar__item sidebar__item--uncomplete",
                            onclick: move |_|{},
                            IconButton {
                                class: "button--avatar",
                                body: rsx!(
                                    Avatar {
                                        name: "{communities.get_community().name}",
                                        size: 60,
                                        uri: communities.get_community().logo,
                                        variant: Variant::SemiRound
                                    }
                                ),
                                on_click: move |_| { }
                            }
                            span {
                                {communities.get_community().name}
                            }
                        }
                    }
                    for community in communities.get() {
                        li { class: "sidebar__item",
                            onclick: move |_|{},
                            IconButton {
                                class: "button--avatar",
                                body: rsx!(
                                    Avatar {
                                        name: "{community.name}",
                                        size: 60,
                                        uri: community.logo,
                                        variant: Variant::SemiRound
                                    }
                                ),
                                on_click: move |_| { }
                            }
                            span {
                                {community.name}
                            },
                        }
                    }
                    li { class: "sidebar__item",
                        onclick: move |_|{},
                        IconButton {
                            class: "button--icon bg--state-primary-active",
                            size: ElementSize::Big,
                            variant: icon_button::Variant::SemiRound,
                            body: rsx!(
                                Icon {
                                    icon: AddPlus,
                                    height: 32,
                                    width: 32,
                                    stroke_width: 1,
                                    stroke: "var(--text-white)"
                                }
                            ),
                            on_click: move |_| {
                                // nav.push(Route::Discover {});
                            }
                        }
                        span {
                            {translate!(i18, "sidebar.cta")}
                        }
                    }
                    li { class: "sidebar__item",
                        onclick: move |_|{},
                        IconButton {
                            class: "button--icon bg--state-primary-active",
                            size: ElementSize::Big,
                            variant: icon_button::Variant::SemiRound,
                            body: rsx!(
                                Icon {
                                    icon: Compass,
                                    height: 32,
                                    width: 32,
                                    stroke_width: 1,
                                    fill: "var(--text-white)"
                                }
                            ),
                            on_click: move |_| {
                                // nav.push(Route::Discover {});
                            }
                        }
                        span {
                            {translate!(i18, "sidebar.cta")}
                        }
                    }
                    li { class: "sidebar__item",
                        onclick: move |_|{},
                        IconButton {
                            class: "button--icon bg--state-primary-active",
                            size: ElementSize::Big,
                            variant: icon_button::Variant::SemiRound,
                            body: rsx!(
                                Icon {
                                    icon: AddPlus,
                                    height: 32,
                                    width: 32,
                                    stroke_width: 1,
                                    stroke: "var(--text-white)"
                                }
                            ),
                            on_click: move |_| {
                                // nav.push(Route::Discover {});
                            }
                        }
                        span {
                            {translate!(i18, "sidebar.cta")}
                        }
                    }

                }
            }
       }
    )
}
