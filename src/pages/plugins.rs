use crate::{
    components::atoms::{
        dropdown::ElementSize, icon_button::Variant, ArrowRight, BankLine, Icon, IconButton, PlugLine, ScalesLine, SearchInput, SuitcaseLine, TeamLine
    },
    hooks::{
        use_accounts::use_accounts,
        use_communities::use_communities,
        use_notification::use_notification,
        use_our_navigator::use_our_navigator,
        use_tabs::{use_tabs, Tab},
        use_timestamp::use_timestamp,
        use_tooltip::use_tooltip,
    },
    middlewares::{is_chain_available::is_chain_available, is_dao_owner::is_dao_owner},
};
use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

#[derive(PartialEq, Clone)]
pub struct CardTemplate {
    title: String,
    description: String,
    icon: Element,
    nav: String,
}

#[component]
pub fn Plugins(id: u16) -> Element {
    let i18 = use_i18();
    let mut tooltip = use_tooltip();
    let mut communities = use_communities();
    let mut tabs = use_tabs();
    let nav = use_our_navigator();
    let notification = use_notification();
    let accounts = use_accounts();
    let timestamp = use_timestamp();

    use_effect(use_reactive(
        (&communities.get_communities().len(),),
        move |(len,)| {
            if len > 0 {
                if let Err(_) = communities.set_community(id) {
                    let path = format!("/");
                    nav.push(vec![], &path);
                };
            }
        },
    ));

    let cards: Vec<CardTemplate> = vec![
        CardTemplate {
            title: translate!(i18, "plugins.cta_cards.initiatives.title"),
            description: translate!(i18, "plugins.cta_cards.initiatives.description"),
            icon: rsx!(Icon {
                icon: SuitcaseLine,
                height: 56,
                width: 56,
                fill: "var(--fill-600)"
            }),
            nav: "initiatives".to_string(),
        },
        CardTemplate {
            title: translate!(i18, "plugins.cta_cards.members.title"),
            description: translate!(i18, "plugins.cta_cards.members.description"),
            icon: rsx!(Icon {
                icon: TeamLine,
                height: 56,
                width: 56,
                fill: "var(--fill-600)"
            }),
            nav: "members".to_string(),
        },
        CardTemplate {
            title: translate!(i18, "plugins.cta_cards.settings.title"),
            description: translate!(i18, "plugins.cta_cards.settings.description"),
            icon: rsx!(Icon {
                icon: ScalesLine,
                height: 56,
                width: 56,
                fill: "var(--fill-600)"
            }),
            nav: "settings".to_string(),
        },
        CardTemplate {
            title: translate!(i18, "plugins.cta_cards.treasury.title"),
            description: translate!(i18, "plugins.cta_cards.treasury.description"),
            icon: rsx!(Icon {
                icon: BankLine,
                height: 56,
                width: 56,
                fill: "var(--fill-600)"
            }),
            nav: "treasury".to_string(),
        },
    ];

    rsx! {
        div { class: "plugins grid-main",
            div { class: "dashboard__head",
                h3 { class: "dashboard__head__title", {communities.get_community().name}}
                div { class: "head__actions",
                    SearchInput {
                        message: String::new(),
                        placeholder: translate!(i18, "dashboard.cta_header.search"),
                        error: None,
                        on_input: move |_: Event<FormData>| {

                        },
                        on_keypress: move |_| {},
                        on_click: move |_| {}
                    }
                }
            }
            div { class: "dashboard__communities",
                for card in cards {
                    section { class: "card",
                        div { class: "card__container",
                            {card.icon}
                            div { class: "card__head",
                                h3 { class: "card__title",
                                    {card.title}
                                }
                            }
                            p { class: "card__description",
                                {card.description}
                            }
                        }
                        div { class: "card__cta",
                            IconButton {
                                class: "button--avatar",
                                variant: Variant::Secondary,
                                size: ElementSize::Medium,
                                body: rsx!(
                                    Icon { icon : ArrowRight, height : 24, width : 24, fill : "var(--state-primary-active)" }
                                ),
                                on_click: move |_| {
                                    tooltip.hide();
                                    let path = format!("/dao/{}/{}", id, card.nav);
                                    tabs.update(Tab { name: communities.get_community().name, path: path.clone() });
                                    nav.push(
                                        vec![
                                            Box::new(is_chain_available(i18, timestamp, notification)),
                                            Box::new(is_dao_owner(i18, accounts, notification)),
                                        ],
                                        &path,
                                    );
                                }
                            }
                        }
                    }
                }

                section { class: "card card--reverse card--comming-soon",
                    div { class: "card__container",
                        Icon {
                            icon: PlugLine,
                            height: 56,
                            width: 56,
                            stroke_width: 1,
                            fill: "var(--fill-600)"
                        }
                        div { class: "card__head",
                            h3 { class: "card__title",
                                { translate!(i18, "plugins.cta_cards.discover_plugins.title") }
                            }
                        }
                        p { class: "card__description",
                            { translate!(i18, "plugins.cta_cards.discover_plugins.title") }
                        }
                    }
                    div { class: "card__cta",
                        IconButton {
                            class: "button--avatar",
                            variant: Variant::Secondary,
                            size: ElementSize::Medium,
                            body: rsx!(
                                Icon { icon : ArrowRight, height : 24, width : 24, fill : "var(--state-primary-active)" }
                            ),
                            on_click: move |_| {
                                tooltip.hide();
                                nav.push(
                                    vec![
                                        Box::new(is_chain_available(i18, timestamp, notification)),
                                        Box::new(is_dao_owner(i18, accounts, notification)),
                                    ],
                                    "/onboarding",
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}
