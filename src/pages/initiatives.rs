use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use futures_util::StreamExt;

use crate::{
    components::{
        atoms::{
            avatar::Variant as AvatarVariant, dropdown::ElementSize, icon_button::Variant,
            input::InputType, AddPlus, ArrowLeft, ArrowRight, Avatar, Badge, Chat, Icon,
            IconButton, SearchInput, Suitcase, Tab, UserGroup,
        },
        molecules::tabs::TabItem,
    },
    hooks::{
        use_accounts::use_accounts, use_notification::use_notification,
        use_our_navigator::use_our_navigator, use_tooltip::use_tooltip,
    },
    middlewares::is_dao_owner::is_dao_owner,
    pages::dashboard::Community,
};

static SKIP: u8 = 6;

#[component]
pub fn Initiatives(id: u16) -> Element {
    let i18 = use_i18();
    let mut notification = use_notification();
    let mut tooltip = use_tooltip();
    let mut nav = use_our_navigator();
    let accounts = use_accounts();

    let header_handled = consume_context::<Signal<bool>>();

    let mut current_page = use_signal::<u8>(|| 1);
    let mut search_word = use_signal::<String>(|| String::new());

    let tab_items = vec![TabItem {
        k: String::from("all"),
        value: translate!(i18, "dao.tabs.all"),
    }];

    let mut tab_value = use_signal::<String>(|| String::from("all"));

    let mut communities_ids = use_signal::<Vec<u16>>(|| vec![]);
    let mut communities = use_signal::<Vec<Community>>(|| vec![]);
    let mut filtered_communities = use_signal::<Vec<Community>>(|| vec![]);

    // let mut items = vec![];
    // for item in tab_items.into_iter() {
    //     items.push(rsx!(Tab {
    //         text: item.value,
    //         on_click: move |_| {
    //             // tab_value.set(item.k);
    //         },
    //     }))
    // }

    rsx! {
        div {
            class: "dashboard grid-main",
            div { class: "dashboard__head",
                section { class: "tabs",
                    // body: items
                    for item in tab_items.into_iter() {
                        Tab {
                            text: item.value,
                            is_active: if tab_value() == item.k {true} else {false},
                            on_click: move |_| {
                                // tab_value.set(item.k);
                            },
                        }
                    }
                }
                div { class: "head__actions",
                    SearchInput {
                        message: search_word(),
                        itype: InputType::Search,
                        placeholder: translate!(i18, "dao.cta_header.search"),
                        error: None,
                        on_input: move |event: Event<FormData>| {
                            search_word.set(event.value());

                            if search_word().trim().is_empty() {
                                filtered_communities.set(communities());
                            } else {
                                let pattern = search_word().trim().to_lowercase();
                                filtered_communities.set(communities().into_iter().filter(|community| community.name.to_lowercase().contains(&pattern)).collect::<Vec<Community>>());
                            }
                        },
                        on_keypress: move |_| {},
                        on_click: move |_| {},
                    }
                    IconButton {
                        class: "button--avatar desktop",
                        size: ElementSize::Medium,
                        body: rsx!(
                            Icon {
                                icon: AddPlus,
                                height: 26,
                                width: 26,
                                stroke_width: 1.5,
                                fill: "var(--fill-00)"
                            }
                        ),
                        on_click: move |_| {
                            tooltip.hide();
                            nav.push(vec![Box::new(is_dao_owner())], "/onboarding");
                        }
                    }
                }
            }
            div { class: "dashboard__communities",
                for community in filtered_communities() {
                    section { class: "card",
                        div { class: "card__container",
                            div { class: "card__head",
                                IconButton {
                                    body: rsx!(
                                        Avatar {
                                            name: "{community.name}",
                                            size: 48,
                                            uri: community.icon,
                                            variant: AvatarVariant::SemiRound
                                        }
                                    ),
                                    on_click: move |_| { }
                                }
                                h3 { class: "card__title",
                                    "{community.name}"
                                }
                            }
                            p { class: "card__description",
                                "{community.description}"
                            }
                            div { class: "card__metrics",
                                span { class: "card__metric",
                                    Icon {
                                        icon: UserGroup,
                                        height: 16,
                                        width: 16,
                                        stroke_width: 1,
                                        fill: "var(--text-primary)"
                                    }
                                    small {
                                        "{community.memberships} Memberships"
                                    }
                                }
                                span { class: "card__metric",
                                    Icon {
                                        icon: Suitcase,
                                        height: 16,
                                        width: 16,
                                        stroke_width: 1,
                                        fill: "var(--text-primary)"
                                    }
                                    small {
                                        "{community.members} Members"
                                    }
                                }
                            }
                            div { class: "card__tags",
                                for tag in community.tags {
                                    {
                                        rsx!(
                                            Badge {
                                                class: "badge--lavanda-dark",
                                                text: tag
                                            }
                                        )
                                    }
                                }
                            }
                        }

                        div { class: "card__cta",
                            IconButton {
                                class: "button--avatar buttom--comming-soon",
                                body: rsx!(
                                    Icon {
                                        icon: ArrowRight,
                                        height: 32,
                                        width: 32,
                                        stroke_width: 2,
                                        fill: "var(--fill-00)"
                                    }
                                ),
                                on_click: move |_| { }
                            }
                        }
                    }
                }
                section { class: "card card--reverse card--comming-soon",
                    div { class: "card__container",
                        div { class: "card__head",
                            h3 { class: "card__title",
                                {translate!(i18, "dao.cta_cards.create.title")}
                            }
                        }
                        p { class: "card__description",
                            {translate!(i18, "dao.cta_cards.create.description")}
                        }
                        div { class: "card__head",
                            a { class: "card__learn",
                                {translate!(i18, "dao.cta_cards.create.cta")}
                            }
                            Icon {
                                icon: ArrowRight,
                                height: 20,
                                width: 20,
                                stroke_width: 1,
                                fill: "var(--text-tertiary)"
                            }
                        }
                    }

                    div { class: "card__cta",
                        IconButton {
                            class: "button--avatar",
                            size: ElementSize::Big,
                            body: rsx!(
                                Icon {
                                    icon: AddPlus,
                                    height: 32,
                                    width: 32,
                                    stroke_width: 1.5,
                                    fill: "var(--fill-00)"
                                }
                            ),
                            on_click: move |_| {
                                tooltip.hide();
                            }
                        }
                    }
                }
            }
            div { class: "dashboard__footer grid-footer",
                div { class: "dashboard__footer__pagination",
                    span { class: "dashboard__footer__paginator",
                        {translate!(i18, "dashboard.footer.paginator", from: current_page(), to: (((communities_ids.len() as f64 + 1f64) / SKIP as f64) as f64).ceil())}
                    }
                    div { class: "dashboard__footer__paginators",
                        IconButton {
                            class: "button--avatar",
                            size: ElementSize::Small,
                            body: rsx!(
                                Icon {
                                    icon: ArrowLeft,
                                    height: 24,
                                    width: 24,
                                    fill: "var(--white)"
                                }
                            ),
                            on_click: move |_| {
                                let current = current_page();
                                current_page.set(current - 1);

                                // get_community_track.send(current_page())
                            }
                        }
                        IconButton {
                            class: "button--avatar",
                            size: ElementSize::Small,
                            body: rsx!(
                                Icon {
                                    icon: ArrowRight,
                                    height: 24,
                                    width: 24,
                                    fill: "var(--white)"
                                }
                            ),
                            on_click: move |_| {
                                let current = current_page();
                                current_page.set(current + 1);

                                // get_community_track.send(current_page())
                            }
                        }
                    }
                }
            }
        }
        div { class: "dashboard__floating",
            IconButton {
                variant: Variant::SemiRound,
                size: ElementSize::Big,
                class: "button--avatar",
                body: rsx!(
                    Icon {
                        icon: Chat,
                        height: 32,
                        width: 32,
                        fill: "var(--fill-00)"
                    }
                ),
                on_click: move |_| {
                    // nav.push()
                }
            }
        }
    }
}