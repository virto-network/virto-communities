use crate::{
    components::{
        atoms::{
            avatar::Variant as AvatarVariant, dropdown::ElementSize, AddPlus,
            ArrowRight, Avatar, Badge, CardSkeleton, DynamicText, Icon, IconButton, SearchInput,
            Star, Tab, UserAdd, UserGroup,
        },
        molecules::{paginator::PaginatorValue, tabs::TabItem, Paginator},
    },
    hooks::{

        use_accounts::use_accounts,
        use_communities::{get_cached_communities, use_communities, CommunitiesError},
        use_notification::use_notification,
        use_our_navigator::use_our_navigator,
        use_timestamp::use_timestamp,
        use_tooltip::use_tooltip,
    },
    middlewares::{is_chain_available::is_chain_available, is_dao_owner::is_dao_owner},
};
use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use futures_util::StreamExt;

static SKIP: usize = 7;

#[component]
pub fn Explore() -> Element {
    let i18 = use_i18();
    let mut tooltip = use_tooltip();
    let nav = use_our_navigator();
    let mut communities = use_communities();
    let mut notification = use_notification();
    let accounts = use_accounts();
    let timestamp = use_timestamp();

    let mut current_page = use_signal::<usize>(|| 1);
    let mut search_word = use_signal::<String>(|| String::new());
    let tab_items = vec![TabItem {
        k: String::from("all"),
        value: translate!(i18, "dashboard.tabs.all"),
    }];
    let tab_value = use_signal::<String>(|| String::from("all"));

    let mut filter_name = use_signal::<Option<String>>(|| None);
    let mut filter_paginator = use_signal::<Option<(usize, usize)>>(|| None);

    let on_handle_paginator = use_coroutine(move |mut rx: UnboundedReceiver<usize>| async move {
        while let Some(f) = rx.next().await {
            let from = if f - 1 > 0 { (f - 1) * SKIP } else { 0 };
            let to = f * SKIP;

            filter_paginator.set(Some((from, to)))
        }
    });
    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        on_handle_paginator.send(current_page());
    });

    let cached_communities_len = get_cached_communities().len();

    let dynamic_one = translate!(i18, "dynamic_text.dynamic_one");
    let dynamic_two = translate!(i18, "dynamic_text.dynamic_two");
    let dynamic_three = translate!(i18, "dynamic_text.dynamic_three");

    let words = vec![dynamic_one, dynamic_two, dynamic_three];

    rsx! {
        div { class: "dashboard grid-main",
            div { class: "dashboard__head",
                section { class: "tabs",
                    for item in tab_items.into_iter() {
                        Tab {
                            text: item.value,
                            is_active: if tab_value() == item.k { true } else { false },
                            on_click: move |_| {}
                        }
                    }
                }
                div { class: "head__actions",
                    SearchInput {
                        message: search_word(),
                        placeholder: translate!(i18, "dashboard.cta_header.search"),
                        error: None,
                        on_input: move |event: Event<FormData>| {
                            search_word.set(event.value());
                            if search_word().trim().is_empty() {
                                filter_name.set(None);
                            } else {
                                let pattern = search_word().trim().to_lowercase();
                                filter_name.set(Some(pattern));
                            }
                        },
                        on_keypress: move |_| {},
                        on_click: move |_| {}
                    }
                    IconButton {
                        class: "button--avatar desktop",
                        size: ElementSize::Medium,
                        body: rsx! {
                            Icon {
                                icon: AddPlus,
                                height: 26,
                                width: 26,
                                stroke_width: 1.5,
                                fill: "var(--fill-00)"
                            }
                        },
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
            div { class: "dashboard__communities",
                {
                    if (communities.is_loading)() {
                    rsx! {
                        for _ in 0..cached_communities_len / 2 {
                            CardSkeleton {}
                        }
                        CardSkeleton {}
                    }
                    } else {
                        rsx! {
                            for community in communities
                                .get_communities_by_filters(None, filter_name().as_deref(), filter_paginator())
                            {
                                section { class: "card",
                                    div { class: "card__container",
                                        div { class: "card__head",
                                            IconButton {
                                                body: rsx!(
                                                    Avatar { name : "{community.name}", size : 48, uri : community.icon, variant :
                                                    AvatarVariant::SemiRound }
                                                ),
                                                on_click: move |_| {}
                                            }
                                            h3 { class: "card__title", "{community.name}" }
                                        }
                                        p { class: "card__description", "{community.description}" }
                                        if !community.has_membership {
                                            div { class: "card__favorite",
                                                IconButton {
                                                    class: "button--drop bg--transparent",
                                                    body: rsx!(
                                                        Icon { icon : Star, height : 24, width : 24, fill : if community.favorite {
                                                        "var(--state-primary-active)" } else { "var(--state-base-background)" } }
                                                    ),
                                                    on_click: move |_| {
                                                        if let Err(e) = communities.handle_favorite(community.id) {
                                                            let message = match e {
                                                                CommunitiesError::NotFound => "Failed to update favorite",
                                                                CommunitiesError::FailedUpdatingFavorites => "Failed to update favorite",
                                                                CommunitiesError::NotFoundFavorite => "Failed to update favorite",
                                                            };
                                                            notification.handle_error(&message);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        div { class: "card__metrics",
                                            span { class: "card__metric",
                                                Icon {
                                                    icon: UserAdd,
                                                    height: 16,
                                                    width: 16,
                                                    stroke_width: 2,
                                                    stroke: "var(--text-primary)"
                                                }
                                                small { "{community.memberships} Free Memberships" }
                                            }
                                            span { class: "card__metric",
                                                Icon {
                                                    icon: UserGroup,
                                                    height: 16,
                                                    width: 16,
                                                    stroke_width: 1,
                                                    fill: "var(--text-primary)"
                                                }
                                                small { "{community.members} Members" }
                                            }
                                        }
                                        div { class: "card__tags",
                                            for tag in community.tags {
                                                Badge { class: "badge--lavanda-dark", text: tag }
                                            }
                                        }
                                    }
                                    div { class: "card__cta",
                                        IconButton {
                                            class: "button--avatar buttom--comming-soon",
                                            body: rsx!(
                                                Icon { icon : ArrowRight, height : 32, width : 32, stroke_width : 2, fill :
                                                "var(--fill-00)" }
                                            ),
                                            on_click: move |_| {
                                                let path = format!("/dao/{}/initiatives", community.id);
                                                nav.push(vec![], &path);
                                            }
                                        }
                                    }
                                }
                            }
                            section { class: "card card--reverse",
                                div { class: "card__container",
                                    div { class: "card__head",
                                        h3 { class: "card__title",
                                            {translate!(i18, "dashboard.cta_cards.create.title_part_one")}
                                            span {
                                                DynamicText { words: words }
                                            }
                                            {translate!(i18, "dashboard.cta_cards.create.title_part_two")}
                                        }
                                    }
                                    p { class: "card__description",
                                        { translate!(i18,
                                        "dashboard.cta_cards.create.description") }
                                    }
                                    div { class: "card__head",
                                        a { class: "card__learn",
                                            { translate!(i18, "dashboard.cta_cards.create.cta") }
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
                                            Icon { icon : AddPlus, height : 32, width : 32, stroke_width : 1.5, fill :
                                            "var(--fill-00)" }
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
            div { class: "dashboard__footer grid-footer",
                Paginator {
                    to: (communities.get_communities().len() + SKIP - 1).saturating_div(SKIP).max(1),
                    on_change: move |event: PaginatorValue| {
                        current_page.set(event.value());
                        on_handle_paginator.send(current_page())
                    }
                }
            }
        }
    }
}
