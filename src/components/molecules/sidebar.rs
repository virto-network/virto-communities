use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use futures_util::StreamExt;
use crate::{
    components::atoms::{
        avatar::Variant, dropdown::ElementSize, icon_button, AddPlus, Avatar, Compass,
        Hamburguer, Home, Icon, IconButton,
    },
    hooks::{
        use_accounts::use_accounts, use_communities::use_communities,
        use_notification::use_notification, use_our_navigator::use_our_navigator,
        use_tooltip::{use_tooltip, TooltipItem},
    },
    middlewares::is_dao_owner::is_dao_owner, pages::dashboard::Community,
    services::kreivo::community_memberships::get_communities_by_member,
};
#[component]
pub fn Sidebar() -> Element {
    let i18 = use_i18();
    let communities = use_communities();
    let accounts = use_accounts();
    let mut tooltip = use_tooltip();
    let mut nav = use_our_navigator();
    let mut notification = use_notification();
    let header_handled = consume_context::<Signal<bool>>();
    let mut is_active = use_signal(|| false);
    let mut communities_by_address = use_signal::<Vec<Community>>(|| vec![]);
    let active_class = if is_active() { "sidebar--active" } else { "" };
    rsx!(
        section { class: "sidebar {active_class}",
            IconButton {
                class: "button--hamburguer",
                body: rsx!(
                    Icon { icon : Hamburguer, height : 30, width : 30, stroke_width : 2, stroke :
                    "var(--text-1)" }
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
                    if !communities.get_community().name.is_empty()
                        || communities.get_community().logo.is_some()
                    {
                        li {
                            class: "sidebar__item sidebar__item--uncomplete",
                            onclick: move |_| {},
                            IconButton {
                                class: "button--avatar",
                                body: rsx!(
                                    Avatar { name : "{communities.get_community().name}", size : 60, uri :
                                    communities.get_community().logo, variant : Variant::SemiRound }
                                ),
                                on_click: move |_| {}
                            }
                            span { { communities.get_community().name } }
                        }
                    }
                    li { class: "sidebar__item", onclick: move |_| {},
                        IconButton {
                            class: "button--icon bg--state-primary-active",
                            size: ElementSize::Big,
                            variant: icon_button::Variant::Round,
                            body: rsx!(
                                Icon { icon : Home, height : 32, width : 32, stroke_width : 1, fill :
                                "var(--fill-00)" }
                            ),
                            on_click: move |_| {
                                nav.push(vec![], "/");
                            }
                        }
                        span { { translate!(i18, "sidebar.cta") } }
                    }
                    li { class: "sidebar__item", onclick: move |_| {},
                        IconButton {
                            class: "button--icon bg--state-primary-active",
                            size: ElementSize::Big,
                            variant: icon_button::Variant::Round,
                            body: rsx!(
                                Icon { icon : Compass, height : 32, width : 32, stroke_width : 1.5, fill :
                                "var(--fill-00)" }
                            ),
                            on_click: move |_| {
                                tooltip.hide();
                                nav.push(vec![Box::new(is_dao_owner())], "/explore");
                            }
                        }
                        span { { translate!(i18, "sidebar.cta") } }
                    }
                    li { class: "sidebar__item", onclick: move |_| {},
                        IconButton {
                            class: "button--icon bg--state-primary-active",
                            size: ElementSize::Big,
                            variant: icon_button::Variant::Round,
                            body: rsx!(
                                Icon { icon : AddPlus, height : 32, width : 32, stroke_width : 1.5, fill :
                                "var(--fill-00)" }
                            ),
                            on_click: move |_| {
                                tooltip.hide();
                                nav.push(vec![Box::new(is_dao_owner())], "/onboarding");
                            }
                        }
                        span {
                            {
                            translate!(i18, "sidebar.cta") }
                        }
                    }
                    hr { class: "sidebar__divider" }
                    for community in communities_by_address.read().iter() {
                        li { class: "sidebar__item", onclick: move |_| {},
                            IconButton {
                                class: "button--avatar",
                                body: rsx!(
                                    Avatar { name : "{community.name}", size : 60, uri : None, variant :
                                    Variant::Round }
                                ),
                                on_click: move |_| {}
                            }
                            span { "{community.name}" }
                        }
                    }
                }
            }
        }
    )
}
