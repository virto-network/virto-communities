use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use futures_util::StreamExt;

use crate::{
    components::{
        atoms::{
            avatar::Variant as AvatarVariant, dropdown::ElementSize, icon_button::Variant, input::InputType, AddPlus, ArrowLeft, ArrowRight, Avatar, Badge, Chat, Icon, IconButton, SearchInput, Suitcase, Tab, UserAdd, UserGroup, CardSkeleton
        },
        molecules::tabs::TabItem,
    },
    hooks::{
        use_notification::use_notification,
        use_our_navigator::use_our_navigator,
        use_tooltip::{use_tooltip, TooltipItem},
    },
    middlewares::is_dao_owner::is_dao_owner,
    pages::dashboard::Community,
    services::kreivo::{
        community_memberships::{collection, get_owned_memberships, item},
        community_track::{tracks, tracksIds},
    },
};

static SKIP: u8 = 7;

#[component]
pub fn Explore() -> Element {
    let i18 = use_i18();
    let mut notification = use_notification();
    let mut tooltip = use_tooltip();
    let mut nav = use_our_navigator();
    let mut is_loading = use_signal::<bool>(|| true);

    let header_handled = consume_context::<Signal<bool>>();

    let mut current_page = use_signal::<u8>(|| 1);
    let mut search_word = use_signal::<String>(|| String::new());

    let tab_items = vec![TabItem {
        k: String::from("all"),
        value: translate!(i18, "dashboard.tabs.all"),
    }];

    let mut tab_value = use_signal::<String>(|| String::from("all"));

    let mut communities_ids = use_signal::<Vec<u16>>(|| vec![]);
    let mut communities = use_signal::<Vec<Community>>(|| vec![]);
    let mut filtered_communities = use_signal::<Vec<Community>>(|| vec![]);

    let get_community_track = use_coroutine(move |mut rx: UnboundedReceiver<u8>| async move {
        while let Some(f) = rx.next().await {
            is_loading.set(true);
            communities.clear();

            let from = if f - 1 > 0 { (f - 1) * SKIP } else { 0 };
            let to = if usize::from(f * SKIP) >= communities_ids.len() {
                communities_ids.len() as u8
            } else {
                f * SKIP
            };
            let range = &communities_ids()[from as usize..to as usize];

            for track in range {
                let response_track = tracks(*track).await;
                let response_collection = collection(*track).await;
                let response_item = item(*track, None).await;

                let collection_items = match response_collection {
                    Ok(ref collection) => {
                        let address = format!("0x{}", hex::encode(collection.owner.clone()));
                        get_owned_memberships(&address).await.unwrap_or(0u16)
                    }
                    Err(_) => 0u16,
                };

                let Ok(track_info) = response_track else {
                    continue;
                };

                let filtered_name = track_info
                    .name
                    .iter()
                    .filter(|b| **b != 0)
                    .cloned()
                    .collect::<Vec<_>>();

                let filtered_name: &[u8] = &filtered_name;

                let item_details = match response_item {
                    Ok(items) => items,
                    Err(_) => 0u16,
                };

                let mut community = Community {
                    id: *track,
                    icon: None,
                    name: String::from_utf8_lossy(filtered_name).to_string(),
                    description: String::from(""),
                    tags: vec![],
                    memberships: collection_items,
                    members: item_details,
                };

                communities.with_mut(|c| c.push(community))
            }
            tooltip.hide();
            is_loading.set(false);
            filtered_communities.set(communities())
        }
    });

    // let mut items = vec![];
    // for item in tab_items.into_iter() {
    //     items.push(rsx!(Tab {
    //         text: item.value,
    //         on_click: move |_| {
    //             // tab_value.set(item.k);
    //         },
    //     }))
    // }

    let get_communities = use_coroutine(move |mut rx: UnboundedReceiver<()>| async move {
        while let Some(_) = rx.next().await {
            is_loading.set(true);
            tooltip.handle_tooltip(TooltipItem {
                title: translate!(i18, "dashboard.tips.loading.title"),
                body: translate!(i18, "dashboard.tips.loading.description"),
                show: true,
            });

            let Ok(community_tracks) = tracksIds().await else {
                notification.handle_error(&translate!(i18, "errors.communities.query_failed"));
                tooltip.hide();
                is_loading.set(false);
                return;
            };

            communities_ids.set(community_tracks.communities);
            get_community_track.send(current_page());
        }
    });

    use_effect(use_reactive(&header_handled(), move |_| {
        if header_handled() {
            get_communities.send(())
        }
    }));

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
                        placeholder: translate!(i18, "dashboard.cta_header.search"),
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
            if is_loading() {
                CardSkeleton {}
            } else {
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
                                        icon: UserAdd,
                                        height: 16,
                                        width: 16,
                                        stroke_width: 2,
                                        stroke: "var(--text-primary)"
                                    }
                                    small {
                                        "{community.memberships} Memberships"
                                    }
                                }
                                span { class: "card__metric",
                                    Icon {
                                        icon: UserGroup,
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
                                on_click: move |_| {
                                    let path = format!("/dao/{}/initiatives", community.id);
                                    nav.push(vec![], &path);
                                }
                            }
                        }
                    }
                }
            }
                section { class: "card card--reverse",
                    div { class: "card__container",
                        div { class: "card__head",
                            h3 { class: "card__title",
                                {translate!(i18, "dashboard.cta_cards.create.title")}
                            }
                        }
                        p { class: "card__description",
                            {translate!(i18, "dashboard.cta_cards.create.description")}
                        }
                        div { class: "card__head",
                            a { class: "card__learn",
                                {translate!(i18, "dashboard.cta_cards.create.cta")}
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
                                nav.push(vec![Box::new(is_dao_owner())], "/onboarding");
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

                                get_community_track.send(current_page())
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

                                get_community_track.send(current_page())
                            }
                        }
                    }
                }
            }
        }
    }
}
