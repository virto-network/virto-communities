use crate::{
    components::{
        atoms::{
            dropdown::ElementSize, AddPlus, ArrowRight, Badge, CardSkeleton, CircleCheck, Icon,
            IconButton, SearchInput, StopSign, Tab,
        },
        molecules::{paginator::PaginatorValue, tabs::TabItem, Paginator},
    },
    hooks::{
        use_communities::use_communities,
        use_initiative::{use_initiative, InitiativeInfoContent},
        use_our_navigator::use_our_navigator,
        use_spaces_client::use_spaces_client,
        use_tooltip::{use_tooltip, TooltipItem},
    },
    services::kreivo::{
        community_referenda::{metadata_of, referendum_count, referendum_info_for, Ongoing},
        preimage::{preimage_for, request_status_for},
    },
};
use dioxus::{logger::tracing::debug, prelude::*};
use dioxus_i18n::t;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
static SKIP: usize = 6;
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InitiativeWrapper {
    pub id: u16,
    pub info: InitiativeInfoContent,
    pub ongoing: Ongoing,
}
#[component]
pub fn Initiatives(id: u16) -> Element {
    
    let mut tooltip = use_tooltip();
    let nav = use_our_navigator();
    let spaces_client = use_spaces_client();
    let mut communities = use_communities();
    let mut initiative_state = use_initiative();

    let mut initiative_wrapper = consume_context::<Signal<Option<InitiativeWrapper>>>();
    let mut current_page = use_signal::<usize>(|| 1);
    let mut search_word = use_signal::<String>(String::new);
    let tab_items = vec![TabItem {
        k: String::from("all"),
        value: t!("dao-tabs-all"),
    }];
    let tab_value = use_signal::<String>(|| String::from("all"));
    let initiatives_ids = use_signal::<Vec<u32>>(Vec::new);
    let mut initiatives = use_signal::<Vec<InitiativeWrapper>>(Vec::new);
    let mut filtered_initiatives = use_signal::<Vec<InitiativeWrapper>>(Vec::new);

    use_effect(use_reactive(
        (&communities.get_communities().len(),),
        move |(len,)| {
            if len > 0 && communities.set_community(id).is_err() {
                nav.push(vec![], "/");
            }
        },
    ));

    let handle_initiatives = use_coroutine(move |mut rx: UnboundedReceiver<u16>| async move {
        while let Some(id) = rx.next().await {
            initiative_state.set_loading(true);
            initiatives.set(vec![]);
            filtered_initiatives.set(vec![]);

            tooltip.handle_tooltip(TooltipItem {
                title: t!("dao-tips-loading-title"),
                body: t!("dao-tips-loading-description"),
                show: true,
            });
            // Temporal value for FIFO ongoing initiative
            let from = 29;

            let count = referendum_count()
                .await
                .expect("Should get referendum count");

            for track in from..count {
                let Ok(response) = referendum_info_for(track).await else {
                    continue;
                };

                if response.ongoing.origin.communities.community_id == id {
                    let name = format!("Ref: {:?}", track);
                    let mut init = InitiativeWrapper {
                        id: track,
                        info: InitiativeInfoContent {
                            name,
                            description: String::new(),
                            tags: vec![],
                            actions: vec![],
                        },
                        ongoing: response.ongoing,
                    };

                    debug!("{:?}", metadata_of(track).await);
                    let Ok(initiative_metadata) = metadata_of(track).await else {
                        initiatives.with_mut(|c| c.push(init));
                        continue;
                    };

                    let initiative_metadata = format!("0x{}", hex::encode(initiative_metadata));

                    let Ok(preimage_len) = request_status_for(&initiative_metadata).await else {
                        continue;
                    };

                    let Ok(room_id_metadata) =
                        preimage_for(&initiative_metadata, preimage_len).await
                    else {
                        initiatives.with_mut(|c| c.push(init));
                        continue;
                    };

                    let Ok(response) = spaces_client
                        .get()
                        .get_initiative_by_id(&room_id_metadata)
                        .await
                    else {
                        initiatives.with_mut(|c| c.push(init));
                        continue;
                    };

                    debug!("{:?}", response.info);

                    init.info = response.info;

                    initiatives.with_mut(|c| c.push(init));
                }
            }

            initiative_state.set_loading(false);
            tooltip.hide();
            filtered_initiatives.set(initiatives());
        }
    });

    use_effect(use_reactive(&id, move |_| handle_initiatives.send(id)));

    use_drop(move || communities.remove_community());

    rsx! {
        div { class: "dashboard grid-main",
            div { class: "dashboard__head",
                section { class: "tabs",
                    for item in tab_items.into_iter() {
                        Tab { text: item.value, is_active: tab_value() == item.k, on_click: move |_| {} }
                    }
                }
                div { class: "head__actions",
                    SearchInput {
                        message: search_word(),
                        placeholder: t!("dao-cta_header-search"),
                        error: None,
                        on_input: move |event: Event<FormData>| {
                            search_word.set(event.value());
                            if search_word().trim().is_empty() {
                                filtered_initiatives.set(initiatives());
                            } else {
                                let pattern = search_word().trim().to_lowercase();
                                filtered_initiatives
                                    .set(
                                        initiatives()
                                            .into_iter()
                                            .filter(|initiative| {
                                                initiative.info.name.to_lowercase().contains(&pattern)
                                            })
                                            .collect::<Vec<InitiativeWrapper>>(),
                                    );
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
                            let path = format!("/dao/{}/initiative", id);
                            nav.push(vec![], &path);
                        }
                    }
                }
            }
            div { class: "dashboard__communities",
                { if initiative_state.is_loading() {
                        rsx! {
                            CardSkeleton {}
                        }
                        } else {
                            rsx! {
                                for initiative in filtered_initiatives() {
                                    section { class: "card",
                                        div { class: "card__container",
                                            div { class: "card__head",
                                                h3 { class: "card__title", "{initiative.info.name}" }
                                            }
                                            p { class: "card__description", "" }
                                            div { class: "card__metrics",
                                                span { class: "card__metric",
                                                    Icon { icon: CircleCheck, height: 16, width: 16, fill: "var(--text-primary)" }
                                                    small { "{initiative.ongoing.tally.ayes} Aye" }
                                                }
                                                span { class: "card__metric",
                                                    Icon {
                                                        icon: StopSign,
                                                        height: 16,
                                                        width: 16,
                                                        stroke_width: 2,
                                                        stroke: "var(--text-primary)"
                                                    }
                                                    small { "{initiative.ongoing.tally.nays} Nay" }
                                                }
                                            }
                                            div { class: "card__tags",
                                                for tag in initiative.clone().info.tags {
                                                    { rsx!(Badge {
                                                    class : "badge--lavanda-dark", text : tag }) }
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
                                                    tooltip.hide();
                                                    initiative_wrapper.set(Some(initiative.clone()));
                                                    let path = format!("/dao/{}/vote/{}", id, initiative.id);
                                                    nav.push(vec![], &path);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                },
                section { class: "card card--reverse",
                    div { class: "card__container",
                        div { class: "card__head",
                            h3 { class: "card__title", {t!("dao-cta_cards-create-title")} }
                        }
                        p { class: "card__description",
                            {
                            t!("dao-cta_cards-create-description") }
                        }
                        div { class: "card__head",
                            a { class: "card__learn", {t!("dao-cta_cards-create-cta")} }
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
                            body: rsx! {
                                Icon {
                                    icon: AddPlus,
                                    height: 32,
                                    width: 32,
                                    stroke_width: 1.5,
                                    fill: "var(--fill-00)"
                                }
                            },
                            on_click: move |_| {
                                tooltip.hide();
                                let path = format!("/dao/{}/initiative", id);
                                nav.push(vec![], &path);
                            }
                        }
                    }
                }
            }
            div { class: "dashboard__footer grid-footer",
                Paginator {
                    from: 1,
                    to: (initiatives_ids.len() + SKIP - 1).saturating_div(SKIP).max(1),
                    value: current_page(),
                    on_change: move |event: PaginatorValue| {
                        current_page.set(event.value());
                    }
                }
            }
        }
    }
}
