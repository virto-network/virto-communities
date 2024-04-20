use dioxus::prelude::*;

use crate::{
    components::atoms::{avatar::Variant, AddPlus, Avatar, Hamburguer, Icon, IconButton},
    hooks::use_communities::use_communities,
    pages::route::Route,
};

#[component]
pub fn Sidebar() -> Element {
    let nav = use_navigator();
    let communities = use_communities();

    let mut is_active = use_signal(|| false);

    let active_class = if is_active() { "sidebar--active" } else { "" };

    rsx!(
       section {
           class: "sidebar",
           div { class: "sidebar__container",
                ul { class: "sidebar__list",
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
                        },
                        on_click: move |_| { }
                    }
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
                }
            }
       }
    )
}
