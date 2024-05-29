use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::atoms::{
        dropdown::{DropdownItem, ElementSize},
        icon_button::Variant,
        AddPlus, ArrowRight, Badge, Chat, Icon, IconButton, MessageInput, Monetization, Plus,
        Suitcase, UserGroup,
    },
    hooks::use_theme::use_theme,
};

pub enum Tag {
    Root,
    SmallSpender,
    MediumSpender,
    BigSpender,
}

pub struct Initiatives {
    title: String,
    description: String,
    treasury: u64,
    memberships: u8,
    members: u8,
    tags: Vec<Tag>,
    can_vote: bool,
}

#[component]
pub fn Initiatives() -> Element {
    let i18 = use_i18();
    let nav = use_navigator();
    let mut theme = use_theme();

    let mut dropdown_value = use_signal::<Option<DropdownItem>>(|| None);
    theme.set_background(String::from("var(--olive-5)"));
    theme.set_text_1(String::from("#4D6860"));
    theme.set_text_2(String::from("var(--text-1)"));
    theme.set_text_3(String::from("var(--olive-100)"));
    theme.set_text_4(String::from(""));

    let initiatives = vec![
        Initiatives {
            title: String::from("Green Spaces Initiative"),
            description: String::from("Create more green spaces in our community to promote environmental sustainability."),
            treasury: 50_000,
            memberships: 5,
            members: 5,
            tags: vec![Tag::Root],
            can_vote: true,
        },
        Initiatives {
            title: String::from("Youth Education Program"),
            description: String::from("Develop a program to provide educational opportunities for young people in our community."),
            treasury: 40_000,
            memberships: 5,
            members: 5,
            tags: vec![Tag::Root, Tag::SmallSpender],
            can_vote: true,
        },
        Initiatives {
            title: String::from("Community Health Clinic"),
            description: String::from("Establish a community health clinic to improve healthcare access for residents."),
            treasury: 100_000,
            memberships: 10,
            members: 10,
            tags: vec![Tag::Root, Tag::BigSpender],
            can_vote: true,
        },
        Initiatives {
            title: String::from("Tech Hub Expansion"),
            description: String::from("Expand our community tech hub to provide more resources for aspiring developers."),
            treasury: 80_000,
            memberships: 8,
            members: 8,
            tags: vec![Tag::Root, Tag::MediumSpender],
            can_vote: true,
        },
        Initiatives {
            title: String::from("Public Art Installation"),
            description: String::from("Commission a public art installation to beautify our community."),
            treasury: 30_000,
            memberships: 3,
            members: 3,
            tags: vec![Tag::Root, Tag::SmallSpender],
            can_vote: true,
        },
    ];

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
        div { class: "initiatives grid-main",
            div { class: "dashboard__head",
                h2 { class: "head__title",
                    "YOUR IMPACT"
                }
                div { class: "head__actions",
                    // Dropdown {
                    //     value: dropdown_value(),
                    //     items: dropdown_items,
                    //     default: None,
                    //     on_change: move |event: DropdownItem| {
                    //         dropdown_value.set(Some(event))
                    //     }
                    // }
                    MessageInput {
                        message: "",
                        placeholder: translate!(i18, "discover.form.custom_industry.placeholder"),
                        error: None,
                        on_input: move |_| {},
                        on_keypress: move |_| {},
                        on_click: move |_| {},
                    }
                    IconButton {
                        class: "button--avatar",
                        body: rsx!(
                            Icon {
                                icon: Plus,
                                height: 26,
                                width: 26,
                                stroke_width: 1.5,
                                stroke: "#B5F1B1"
                            }
                        ),
                        on_click: move |_| { }
                    }
                }
            }
            div { class: "dashboard__communities",
                for initiative in initiatives {
                    section { class: "card",
                        div { class: "card__container",
                            div { class: "card__head",
                                h3 { class: "card__title",
                                    "{initiative.title}"
                                }
                            }
                            p { class: "card__description",
                                "{initiative.description}"
                            }
                            div { class: "card__metrics",
                                span { class: "card__metric",
                                    Icon {
                                        icon: Monetization,
                                        height: 16,
                                        width: 16,
                                        stroke_width: 1,
                                        fill: "#4D6860"
                                    }
                                    small {
                                        {nice_money(initiative.treasury)}
                                    }
                                }
                                span { class: "card__metric",
                                    Icon {
                                        icon: UserGroup,
                                        height: 16,
                                        width: 16,
                                        stroke_width: 1,
                                        fill: "#4D6860"
                                    }
                                    small {
                                        "{initiative.memberships}"
                                    }
                                }
                                span { class: "card__metric",
                                    Icon {
                                        icon: Suitcase,
                                        height: 16,
                                        width: 16,
                                        stroke_width: 1,
                                        fill: "#4D6860"
                                    }
                                    small {
                                        "{initiative.members}"
                                    }
                                }
                            }
                            div { class: "card__tags",
                                for tag in initiative.tags {
                                    {
                                        let (badge_class, badge_text) = match tag {
                                            Tag::Root => ("badge--lavanda-dark", "Root"),
                                            Tag::SmallSpender => ("badge--lavanda-light", "SmallSpender"),
                                            Tag::MediumSpender => ("badge--blue-light", "MediumSpender"),
                                            Tag::BigSpender => ("badge--green-light", "BigSpender"),
                                        };

                                        rsx!(
                                            Badge {
                                                class: badge_class,
                                                text: badge_text
                                            }
                                        )
                                    }
                                }
                            }
                        }

                        div { class: "card__cta",
                            IconButton {
                                class: "button--avatar",
                                body: rsx!(
                                    Icon {
                                        icon: ArrowRight,
                                        height: 16,
                                        width: 16,
                                        stroke_width: 2,
                                        stroke: "#B5F1B1"
                                    }
                                ),
                                on_click: move |_| { }
                            }
                        }
                    }
                }
                section { class: "card card--reverse",
                    div { class: "card__container",
                        div { class: "card__head",
                            h3 { class: "card__title",
                                "Create a DAO"
                            }
                        }
                        p { class: "card__description",
                            "A Decentralized and open governed entity for all of your Ideas."
                        }
                        div { class: "card__head",
                            a { class: "card__learn",
                                "Learn more"
                            }
                            Icon {
                                icon: ArrowRight,
                                height: 20,
                                width: 20,
                                stroke_width: 1,
                                stroke: "#56C95F33"
                            }
                        }
                    }

                    div { class: "card__cta",
                        IconButton {
                            class: "button--avatar",
                            body: rsx!(
                                Icon {
                                    icon: AddPlus,
                                    height: 32,
                                    width: 32,
                                    stroke_width: 1.5,
                                    stroke: "#B5F1B1"
                                }
                            ),
                            on_click: move |_| { }
                        }
                    }
                }
            }
            div { class: "dashboard__floating",
                IconButton {
                    variant: Variant::SemiRound,
                    size: ElementSize::Big,
                    class: "button--avatar bg--state-primary-active",
                    body: rsx!(
                        Icon {
                            icon: Chat,
                            height: 32,
                            width: 32,
                            fill: "var(--white)"
                        }
                    ),
                    on_click: move |_| { }
                }
            }
        }
    }
}

fn nice_money(value: u64) -> String {
    let units = vec!["", "K", "M", "B"];
    let mut l = 0;
    let mut n = value as f64;

    while n >= 1000.0 && l < units.len() - 1 {
        n /= 1000.0;
        l += 1;
    }

    format!(
        "${:.2}{}",
        n,
        if n < 10.0 && l > 0 {
            units[l]
        } else {
            units[l]
        }
    )
}
