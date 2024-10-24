use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::atoms::{
        avatar::Variant, dropdown::ElementSize, icon_button, Avatar, Compass, Hamburguer, Home,
        Icon, IconButton, OnOff, Star,
    },
    hooks::{
        use_accounts::use_accounts, use_communities::use_communities,
        use_our_navigator::use_our_navigator, use_tooltip::use_tooltip,
    },
};
#[component]
pub fn Sidebar() -> Element {
    let i18 = use_i18();
    let mut communities = use_communities();
    let nav = use_our_navigator();
    let mut tooltip = use_tooltip();
    let accounts = use_accounts();

    let mut is_active = use_signal(|| false);

    rsx!(
        section { class: "sidebar", class: if is_active() { "sidebar--active" },
            IconButton {
                class: "button--hamburguer",
                body: rsx! {
                    Icon {
                        icon: Hamburguer,
                        height: 30,
                        width: 30,
                        stroke_width: 2,
                        stroke: "var(--text-1)"
                    }
                },
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
                    match accounts.get_account() {
                        Some(account) => rsx!(
                            li { class: "sidebar__item",
                                onclick: move |_|{},
                                IconButton {
                                    class: "button--avatar",
                                    body: rsx!(
                                        Avatar {
                                            name: "{account.name()}",
                                            size: 60,
                                            uri: None,
                                            variant: Variant::Round
                                        }
                                    ),
                                    on_click: move |_| {
                                        nav.push(vec![], "/account")
                                    }
                                }
                                span {
                                    "{account.name()}"
                                },
                            }
                        ),
                        None => rsx!(
                            li { class: "sidebar__item",
                                onclick: move |_|{},
                                IconButton {
                                    class: "button--icon bg--state-primary-active",
                                    size: ElementSize::Big,
                                    variant: icon_button::Variant::Round,
                                    body: rsx!(
                                        Icon {
                                            icon: OnOff,
                                            height: 32,
                                            width: 32,
                                            stroke_width: 2,
                                            stroke: "var(--fill-00)"
                                        }
                                    ),
                                    on_click: move |_| {
                                        nav.push(vec![], "/login");
                                    }
                                }
                                span {
                                    {translate!(i18, "sidebar.cta")}
                                }
                            }
                        ),
                    },
                    li { class: "sidebar__item", onclick: move |_| {},
                        IconButton {
                            class: "button--icon bg--state-primary-active",
                            size: ElementSize::Big,
                            variant: icon_button::Variant::Round,
                            body: rsx! {
                                Icon { icon: Home, height: 32, width: 32, stroke_width: 1, fill: "var(--fill-00)" }
                            },
                            on_click: move |_| {
                                nav.push(vec![], "/");
                            }
                        }
                        span { {translate!(i18, "sidebar.cta")} }
                    }
                    li { class: "sidebar__item", onclick: move |_| {},
                        IconButton {
                            class: "button--icon bg--state-primary-active",
                            size: ElementSize::Big,
                            variant: icon_button::Variant::Round,
                            body: rsx! {
                                Icon {
                                    icon: Compass,
                                    height: 32,
                                    width: 32,
                                    stroke_width: 1.5,
                                    fill: "var(--fill-00)"
                                }
                            },
                            on_click: move |_| {
                                tooltip.hide();
                                nav.push(vec![], "/explore");
                            }
                        }
                        span { {translate!(i18, "sidebar.cta")} }
                    }
                    hr { class: "sidebar__divider" }

                    for community in communities.get_communities_by_filters(Some(()), None, None) {
                        {
                            let active_community = communities.get_community();
                            let community_id = community.id.clone();
                            rsx!(
                                li {
                                    class: "sidebar__item",
                                    class: if active_community.id == community.id { "sidebar__item--active" },
                                    onclick: move |_| {
                                        if let Ok(_) = communities.set_community(community_id) {
                                            let path = format!("/dao/{}/initiatives", community_id);
                                            nav.push(vec![], &path);
                                        };
                                    },
                                    if community.favorite {
                                        div { class: "sidebar__item--favorite",
                                            Icon {
                                                icon: Star,
                                                height: 16,
                                                width: 16,
                                                fill: "var(--state-base-background)"
                                            }
                                        }
                                    }
                                    IconButton {
                                        class: "button--avatar",
                                        body: rsx!(
                                            Avatar {
                                                name: "{community.name}",
                                                size: 60,
                                                uri: None,
                                                variant: Variant::Round
                                            }
                                        ),
                                        on_click: move |_| { }
                                    }
                                    span {
                                        "{community.name}"
                                    },
                                }
                            )
                        }
                    }
                }
            }
        }
    )
}
