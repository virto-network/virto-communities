use std::str::FromStr;
use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use futures_util::{StreamExt, TryFutureExt};
use crate::{
    components::{
        atoms::{
            button::Variant, dropdown::ElementSize,
            key_value::Variant as KeyValueVariant, ActionRequest, Badge, Bar, Button,
            CircleCheck, Icon, KeyValue, StopSign,
        },
        molecules::ActionRequestList,
    },
    hooks::{
        use_initiative::{
            ActionItem, InitiativeInfoContent, InitiativeVoteData, Vote, VoteOf,
        },
        use_notification::use_notification, use_our_navigator::use_our_navigator,
        use_session::use_session, use_spaces_client::use_spaces_client,
        use_tooltip::{use_tooltip, TooltipItem},
        use_vote::{ProposalStatus, VoteDigest},
    },
    pages::initiatives::InitiativeWrapper,
    services::kreivo::{
        community_memberships::{get_membership_id, is_community_member_by_address, item},
        community_referenda::{metadata_of, referendum_info_for, Deciding},
        community_track::{tracks, TrackInfo},
        preimage::{preimage_for, request_status_for},
        system::number,
    },
};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = window, js_name = initiativeVote)]
    async fn topup_then_initiative_vote(
        membershipId: u16,
        pollIndex: u16,
        vote: bool,
    ) -> Result<JsValue, JsValue>;
}
#[component]
pub fn Vote(id: u16, initiativeid: u16) -> Element {
    let i18 = use_i18();
    let session = use_session();
    let spaces_client = use_spaces_client();
    let nav = use_our_navigator();
    let mut notification = use_notification();
    let mut tooltip = use_tooltip();
    let mut votes_statistics = use_signal(|| VoteDigest::default());
    let mut content = use_signal(|| String::new());
    let mut can_vote = use_signal(|| false);
    let mut show_requests = use_signal(|| false);
    let mut show_vote = use_signal(|| true);
    let mut initiative_wrapper = consume_context::<Signal<Option<InitiativeWrapper>>>();
    let mut current_block = use_signal(|| 0);
    let mut track_info = use_signal(|| None);
    let mut members = use_signal(|| 0);
    let mut room_id = use_signal(|| None);
    let mut approval_threshold = use_signal(|| 100.0);
    let mut participation_threshold = use_signal(|| 100.0);
    let cont = &*content.read();
    let parser = pulldown_cmark::Parser::new(cont);
    let mut html_buf = String::new();
    pulldown_cmark::html::push_html(&mut html_buf, parser);
    let on_handle_vote = use_coroutine(move |mut rx: UnboundedReceiver<()>| async move {
        while let Some(_) = rx.next().await {
            let Some(account) = session.get() else {
                log::info!("error here by account");
                notification
                    .handle_error(&translate!(i18, "errors.communities.query_failed"));
                return;
            };
            let Ok(address) = sp_core::sr25519::Public::from_str(&account.address) else {
                log::info!("error here by address");
                notification
                    .handle_error(&translate!(i18, "errors.wallet.account_address"));
                return;
            };

            let Ok(is_member) = is_community_member_by_address(&address.0, id).await else {
                log::info!("error here by memeber");
                notification
                    .handle_error(&translate!(i18, "errors.communities.query_failed"));
                return;
            };

            // get community members
            let response_item = item(id).await;
            let item_details = match response_item {
                Ok(items) => items,
                Err(_) => 0u16,
            };
            members.set(item_details);
            let Ok(block) = number().await else {
                log::warn!("Failed to get last block kusama");
                continue;
            };
            current_block.set(block);
            let Ok(track) = tracks(id).await else {
                log::warn!("Failed to get track");
                continue;
            };
            track_info.set(Some(track));
            if initiative_wrapper().is_none() {
                let Ok(response) = referendum_info_for(initiativeid).await else {
                    continue;
                };
                let name = format!("Ref: {:?}", initiativeid);
                initiative_wrapper
                    .set(
                        Some(InitiativeWrapper {
                            id: initiativeid,
                            info: InitiativeInfoContent {
                                name,
                                description: String::new(),
                                tags: vec![],
                                actions: vec![],
                            },
                            ongoing: response.ongoing,
                        }),
                    );
            }
            let threshold = get_approval_threshold(
                &*track_info.read(),
                &initiative_wrapper.unwrap().ongoing.deciding,
                current_block(),
            );
            approval_threshold.set(threshold);
            let threshold = get_participation_threshold(
                &*track_info.read(),
                &initiative_wrapper.unwrap().ongoing.deciding,
                current_block(),
            );
            participation_threshold.set(threshold);

            can_vote.set(is_member);

            if let Some(wrapper) = initiative_wrapper() {
                votes_statistics.set(VoteDigest::default());
                votes_statistics
                    .with_mut(|votes| votes.aye = wrapper.ongoing.tally.ayes);
                votes_statistics
                    .with_mut(|votes| votes.nay = wrapper.ongoing.tally.nays);
            }
            if let Some(mut wrapper) = initiative_wrapper() {
                let Ok(initiative_metadata) = metadata_of(initiativeid).await else {
                    content.set(wrapper.info.description);
                    continue;
                };
                let initiative_metadata = format!(
                    "0x{}",
                    hex::encode(initiative_metadata),
                );
                let Ok(preimage_len) = request_status_for(&initiative_metadata).await
                else {
                    continue;
                };
                let Ok(room_id_metadata) = preimage_for(
                        &initiative_metadata,
                        preimage_len,
                    )
                    .await else {
                    continue;
                };
                log::info!("{}", room_id_metadata);
                let Ok(response) = spaces_client
                    .get()
                    .get_initiative_by_id(&room_id_metadata)
                    .await else {
                    content.set(wrapper.info.description);
                    continue;
                };
                room_id.set(Some(room_id_metadata));
                content.set(response.info.description.clone());
                wrapper.info = response.info.clone();
                initiative_wrapper.set(Some(wrapper.clone()));
            }
        }
    });
    let handle_vote = move |is_vote_aye: bool| {
        spawn(
            async move {
                tooltip
                    .handle_tooltip(TooltipItem {
                        title: translate!(i18, "governance.tips.voting.title"),
                        body: translate!(i18, "governance.tips.voting.description"),
                        show: true,
                    });
                let account_address = session
                    .get()
                    .ok_or(translate!(i18, "errors.wallet.account_address"))?
                    .address;
                let address = sp_core::sr25519::Public::from_str(&account_address)
                    .map_err(|e| {
                        log::warn!("Not found public address: {}", e);
                        translate!(i18, "errors.wallet.account_address")
                    })?;
                let hex_address = hex::encode(address.0);
                let membership_id = get_membership_id(&format!("0x{}", hex_address), id)
                    .await
                    .map_err(|_| translate!(i18, "errors.wallet.account_address"))?;
                if let Some(room_id) = room_id() {
                    spaces_client
                        .get()
                        .vote_initiative(InitiativeVoteData {
                            user: account_address,
                            room: room_id,
                            vote: Vote::Standard(
                                if is_vote_aye { VoteOf::Yes } else { VoteOf::No },
                            ),
                        })
                        .await
                        .map_err(|e| {
                            log::warn!("Failed to persist vote: {:?}", e);
                            translate!(i18, "errors.vote.persist_failed")
                        })?;
                }
                topup_then_initiative_vote(membership_id, initiativeid, is_vote_aye)
                    .await
                    .map_err(|e| {
                        log::warn!("Failed to vote on-chain: {:?}", e);
                        translate!(i18, "errors.vote.chain")
                    })?;
                on_handle_vote.send(());
                tooltip.hide();
                notification
                    .handle_success(
                        &translate!(i18, "governance.tips.voted.description"),
                    );
                let path = format!("/dao/{id}/initiatives");
                nav.push(vec![], &path);
                Ok::<(), String>(())
            }
                .unwrap_or_else(move |e: String| {
                    tooltip.hide();
                    notification.handle_error(&e);
                }),
        );
    };
    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        on_handle_vote.send(())
    });
    rsx! {
        div { class: "page--vote",
            div { class: "initiative__form",
                if let Some(initiative_wrapper) = &*initiative_wrapper.read() {
                    div { class: "form__wrapper form__wrapper--initiative",
                        h2 { class: "form__title", "{initiative_wrapper.info.name}" }
                        div { class: "details__metadata",
                            KeyValue {
                                class: "key-value",
                                text: format!("{}: ", translate!(i18, "governance.description.details.by")),
                                size: ElementSize::Medium,
                                variant: KeyValueVariant::Secondary,
                                body: rsx!(
                                    { let hex_string = hex::encode(& initiative_wrapper.ongoing.submission_deposit
                                    .who); format!("0x{}", hex_string) }
                                )
                            }
                        }
                        div { class: "steps__wrapper",
                            div { class: "row",
                                section { class: "details__voting",
                                    div { class: "vote-card",
                                        h4 { class: "vote-card__title", "Request" }
                                        button {
                                            class: "button--tertiary",
                                            onclick: move |_| show_requests.toggle(),
                                            ActionRequest {
                                                name: if show_requests() { "Hide all requests" } else { "See all requests" },
                                                details: initiative_wrapper
                                                    .info
                                                    .actions
                                                    .iter()
                                                    .map(|item| {
                                                        match item {
                                                            ActionItem::AddMembers(action) => action.members.len(),
                                                            ActionItem::KusamaTreasury(action) => action.periods.len(),
                                                            ActionItem::VotingOpenGov(action) => action.proposals.len(),
                                                            ActionItem::CommunityTransfer(action) => action.transfers.len()
                                                        }
                                                    })
                                                    .sum::<usize>()
                                                    .to_string(),
                                                size: ElementSize::Small
                                            }
                                        }
                                        if show_requests() {
                                            ActionRequestList { actions: initiative_wrapper.info.actions.clone() }
                                        }
                                    }
                                }
                                section { class: "details__voting",
                                    div { class: "vote-card",
                                        div { class: "details__statistics",
                                            div { class: "details__head",
                                                h2 { class: "vote-card__title statistics__title",
                                                    {translate!(i18, "governance.description.details.status.title")}
                                                }
                                                {
                                                    let status = if initiative_wrapper.ongoing.in_queue | initiative_wrapper.ongoing.deciding.is_none() {
                                                        ProposalStatus::QUEUE
                                                    } else {
                                                        ProposalStatus::VOTING
                                                    };
                                                    let (badge_title, badge_color) = match status {
                                                        ProposalStatus::APPROVED => (translate!(i18, "governance.description.details.status.options.approved"), "badge--green-dark"),
                                                        ProposalStatus::REJECTED => (translate!(i18, "governance.description.details.status.options.rejected"), "badge--red-dark"),
                                                        ProposalStatus::VOTING => (translate!(i18, "governance.description.details.status.options.voting"), "badge--lavanda-dark"),
                                                        ProposalStatus::QUEUE => (translate!(i18, "governance.description.details.status.options.queue"), "badge--blue-light"),
                                                    };
                                                
                                                    rsx!(
                                                        Badge {
                                                            text: badge_title,
                                                            class: badge_color.to_string()
                                                        }
                                                    )
                                                }
                                            }
                                            div {
                                                {
                                                    let mut consumed = 0;
                                                
                                                    if let Some(deciding) = &initiative_wrapper.ongoing.deciding {
                                                        if  current_block() > 0 {
                                                            consumed = current_block() - deciding.since;
                                                        }
                                                    }
                                                
                                                    let decision = match &*track_info.read() {
                                                        Some(info) => info.decision_period,
                                                        None => 36000
                                                    };
                                                
                                                    let consumed_percent = 100.0 / decision as f64 * consumed as f64;
                                                
                                                    rsx!(
                                                        Bar {
                                                            left_value: consumed_percent,
                                                            right_value: 100.0 - consumed_percent,
                                                            right_helper: if blocks_to_days(decision - consumed) == 0 {
                                                                format!("{}", blocks_to_days(decision - consumed) + 1)
                                                            } else {
                                                                format!("{}", blocks_to_days(decision - consumed))
                                                            },
                                                            left_title: "Decision",
                                                            right_title: match blocks_to_times(decision) {
                                                                Times::Minutes(time) => {format!("{} Minutes", time)},
                                                                Times::Hours(time) => {format!("{} Hours", time)},
                                                                Times::Days(time) => {format!("{} Days", time)},
                                                            },
                                                        }
                                                    )
                                                }
                                            }
                                        }
                                    }
                                }
                                section { class: "details__voting",
                                    div { class: "vote-card",
                                        div { class: "details__statistics",
                                            div { class: "details__head",
                                                h2 { class: "vote-card__title statistics__title",
                                                    { translate!(i18,
                                                    "governance.description.voting.title") }
                                                }
                                                Button {
                                                    text: if show_vote() { "Hide vote" } else { "Vote" },
                                                    size: ElementSize::Small,
                                                    variant: Variant::Secondary,
                                                    on_click: move |_| {
                                                        show_vote.toggle();
                                                    },
                                                    status: None
                                                }
                                            }
                                            if show_vote() {
                                                div { class: "note",
                                                    "Explain that this is a dynamic voting, and thresholds might change."
                                                }
                                            }
                                            if show_vote() && can_vote() {
                                                div { class: "row",
                                                    Button {
                                                        class: "vote-cta",
                                                        text: translate!(i18, "governance.description.voting.cta.for"),
                                                        size: ElementSize::Medium,
                                                        variant: Variant::Secondary,
                                                        on_click: move |_| { handle_vote(true) },
                                                        status: None,
                                                        left_icon: rsx!(Icon { icon : CircleCheck, height : 16, width : 16, fill : "#56C95F" })
                                                    }
                                                    Button {
                                                        class: "vote-cta",
                                                        text: translate!(i18, "governance.description.voting.cta.against"),
                                                        size: ElementSize::Medium,
                                                        variant: Variant::Secondary,
                                                        on_click: move |_| { handle_vote(false) },
                                                        status: None,
                                                        left_icon: rsx!(
                                                            Icon { icon : StopSign, height : 16, width : 16, stroke_width : 2, stroke :
                                                            "#f44336bd" }
                                                        )
                                                    }
                                                }
                                            }
                                            Bar {
                                                left_value: votes_statistics().percent_aye(),
                                                center_value: approval_threshold(),
                                                right_value: votes_statistics().percent_nay(),
                                                left_helper: translate!(i18, "governance.description.voting.for"),
                                                right_helper: translate!(i18, "governance.description.voting.against"),
                                                left_title: format!("{:.1}%", votes_statistics().percent_aye()),
                                                right_title: format!("{:.1}%", votes_statistics().percent_nay()),
                                                variant: crate::components::atoms::bar::Variant::Vote
                                            }
                                            if show_vote() {
                                                div { class: "note",
                                                    KeyValue {
                                                        class: "key-value--row",
                                                        text: "Threshold",
                                                        size: ElementSize::Medium,
                                                        body: rsx!({ format!("{:.1}%", approval_threshold()) })
                                                    }
                                                    KeyValue {
                                                        class: "key-value--row",
                                                        text: "Current approval",
                                                        size: ElementSize::Medium,
                                                        body: rsx!({ format!("{:.1}%", votes_statistics().percent_aye()) })
                                                    }
                                                }
                                            }
                                            if show_vote() {
                                                div {
                                                    div { class: "votes-counter votes-counter--for",
                                                        Icon { icon: CircleCheck, height: 16, width: 16, stroke_width: 2, fill: "#56C95F" }
                                                        p { class: "votes-counter__total",
                                                            "{votes_statistics().aye} "
                                                            {translate!(i18, "governance.description.voting.votes")}
                                                        }
                                                    }
                                                    div { class: "votes-counter votes-counter--against",
                                                        Icon { icon: StopSign, height: 16, width: 16, stroke_width: 2, stroke: "#f44336bd" }
                                                        p { class: "votes-counter__total",
                                                            "{votes_statistics().nay} "
                                                            {translate!(i18, "governance.description.voting.votes")}
                                                        }
                                                    }
                                                }
                                                div {
                                                    { let consumed_percent =
                                                    100.0 / members() as f64 * votes_statistics().total() as f64; rsx!(Bar {
                                                    left_value : consumed_percent, center_value : participation_threshold(),
                                                    right_value : 100.0 - consumed_percent, left_helper : "Participation", left_title
                                                    : "{votes_statistics().total()}", right_title : "{members()}", }) }
                                                }
                                                div { class: "note",
                                                    KeyValue {
                                                        class: "key-value--row",
                                                        text: "Paricipation threshold",
                                                        size: ElementSize::Medium,
                                                        body: rsx!({ format!("{:.1}%", participation_threshold()) })
                                                    }
                                                    KeyValue {
                                                        class: "key-value--row",
                                                        text: "Current support",
                                                        size: ElementSize::Medium,
                                                        body: rsx!(
                                                            { let consumed_percent = 100.0 / members() as f64 * votes_statistics().total() as
                                                            f64; format!("{:.1}%", consumed_percent) }
                                                        )
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            section { class: "details__proposal",
                                div { class: "details__subtitle", "Content" }
                                div { class: "details__tags",
                                    div { class: "card__tags",
                                        for tag in initiative_wrapper.clone().info.tags {
                                            Badge { class: "badge--lavanda-dark", text: tag }
                                        }
                                    }
                                }
                                div {
                                    class: "details__description markdown-preview",
                                    dangerous_inner_html: "{html_buf}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
enum Times {
    Minutes(u32),
    Hours(u32),
    Days(u32),
}
fn blocks_to_times(blocks: u32) -> Times {
    let seconds = blocks * 12;
    let minutes = seconds / 60;
    log::info!("minutes {}", minutes);
    if minutes / (24 * 60) > 0 {
        Times::Days(minutes / (24 * 60))
    } else if minutes / 60 > 0 {
        Times::Hours(minutes / 60)
    } else {
        Times::Minutes(minutes)
    }
}
fn blocks_to_days(blocks: u32) -> u32 {
    let seconds = blocks * 12;
    let minutes = seconds / 60;
    minutes / (24 * 60)
}
fn calculate_threshold<F>(
    track_info: &Option<TrackInfo>,
    deciding: &Option<Deciding>,
    current_block: u32,
    threshold_fn: F,
) -> f64
where
    F: Fn(&TrackInfo, f64) -> f64,
{
    let Some(info) = track_info else { return 100.0 };
    let Some(deciding) = deciding else {
        return 100.0;
    };
    if current_block == 0 {
        return 100.0;
    }
    let consumed = current_block - deciding.since;
    let progress = consumed as f64 / 36000.0;
    threshold_fn(info, progress)
}
fn get_approval_threshold(
    track_info: &Option<TrackInfo>,
    deciding: &Option<Deciding>,
    current_block: u32,
) -> f64 {
    calculate_threshold(
        track_info,
        deciding,
        current_block,
        |info, progress| { info.min_approval.calculate_threshold(progress) },
    )
}
fn get_participation_threshold(
    track_info: &Option<TrackInfo>,
    deciding: &Option<Deciding>,
    current_block: u32,
) -> f64 {
    calculate_threshold(
        track_info,
        deciding,
        current_block,
        |info, progress| { info.min_support.calculate_threshold(progress) },
    )
}
