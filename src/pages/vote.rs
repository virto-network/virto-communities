use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use futures_util::{StreamExt, TryFutureExt};

use crate::{
    components::atoms::{
        button::Variant, dropdown::ElementSize, key_value::Variant as KeyValueVariant, Badge,
        Button, CircleCheck, Icon, KeyValue, Request, StopSign,
    },
    hooks::{
        use_initiative::{
            ActionItem, ConvictionVote, InitiativeInfoContent, InitiativeVoteData, Vote, VoteOf,
            VoteType,
        },
        use_notification::use_notification,
        use_our_navigator::use_our_navigator,
        use_session::use_session,
        use_spaces_client::use_spaces_client,
        use_tooltip::{use_tooltip, TooltipItem},
    },
    pages::initiatives::InitiativeWrapper,
    services::kreivo::{
        community_memberships::{get_communities_by_member, get_membership_id, item},
        community_referenda::{metadata_of, referendum_info_for},
        community_track::{tracks, Curve},
        preimage::{preimage_for, request_status_for},
        system::number,
    },
};
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug)]
pub enum InitiativeStep {
    Info,
    Actions,
    Settings,
    Confirmation,
    None,
}

#[derive(Clone, Debug)]
pub enum ProposalStatus {
    APPROVED,
    REJECTED,
    VOTING,
    QUEUE,
}

#[derive(Clone, Debug)]
pub enum BadgeColor {
    YELLOW,
    RED,
    GREEN,
}

#[derive(Clone, Debug, Default)]
pub struct VoteDigest {
    pub aye: u64,
    pub nay: u64,
}

impl VoteDigest {
    fn total(&self) -> u64 {
        self.aye + self.nay
    }

    fn percent_aye(&self) -> f64 {
        if self.total() > 0 {
            let percent_unit = 100.0 / self.total() as f64;
            percent_unit * self.aye as f64
        } else {
            50.0
        }
    }

    fn percent_nay(&self) -> f64 {
        if self.total() > 0 {
            let percent_unit = 100.0 / self.total() as f64;
            percent_unit * self.nay as f64
        } else {
            50.0
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = window, js_name = topupThenInitiativeVote)]
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
    let mut show_vote = use_signal(|| false);

    let mut initiative_wrapper = consume_context::<Signal<Option<InitiativeWrapper>>>();
    let mut current_block = use_signal(|| 0);
    let mut track_info = use_signal(|| None);
    let mut members = use_signal(|| 0);
    let mut room_id = use_signal(|| None);

    let cont = &*content.read();
    let parser = pulldown_cmark::Parser::new(cont);

    let mut html_buf = String::new();
    pulldown_cmark::html::push_html(&mut html_buf, parser);

    let on_handle_vote = use_coroutine(move |mut rx: UnboundedReceiver<()>| async move {
        while let Some(_) = rx.next().await {
            let Some(account) = session.get() else {
                log::info!("error here by account");
                notification.handle_error(&translate!(i18, "errors.communities.query_failed"));

                return;
            };

            let Ok(address) = sp_core::sr25519::Public::from_str(&account.address) else {
                log::info!("error here by address");
                notification.handle_error(&translate!(i18, "errors.wallet.account_address"));

                return;
            };

            let Ok(community_tracks) = get_communities_by_member(&address.0).await else {
                log::info!("error here by memeber");
                notification.handle_error(&translate!(i18, "errors.communities.query_failed"));

                return;
            };

            // get community members
            let response_item = item(id, None).await;
            let item_details = match response_item {
                Ok(items) => items,
                Err(_) => 0u16,
            };

            members.set(item_details);

            // get current block
            let Ok(block) = number().await else {
                log::warn!("Failed to get last block kusama");
                continue;
            };

            current_block.set(block);

            // get track
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
                initiative_wrapper.set(Some(InitiativeWrapper {
                    id: initiativeid,
                    info: InitiativeInfoContent {
                        name,
                        description: String::new(),
                        tags: vec![],
                        actions: vec![],
                    },
                    ongoing: response.ongoing,
                }))
            };

            if let Some(wrapper) = initiative_wrapper() {
                votes_statistics.set(VoteDigest::default());
                votes_statistics.with_mut(|votes| votes.aye = wrapper.ongoing.tally.ayes);
                votes_statistics.with_mut(|votes| votes.nay = wrapper.ongoing.tally.nays);
            }

            if let Some(mut wrapper) = initiative_wrapper() {
                let Ok(initiative_metadata) = metadata_of(initiativeid).await else {
                    content.set(wrapper.info.description);
                    continue;
                };

                let initiative_metadata = format!("0x{}", hex::encode(initiative_metadata));

                let Ok(preimage_len) = request_status_for(&initiative_metadata).await else {
                    continue;
                };

                let Ok(room_id_metadata) = preimage_for(&initiative_metadata, preimage_len).await
                else {
                    continue;
                };

                log::info!("{}", room_id_metadata);

                let Ok(response) = spaces_client
                    .get()
                    .get_initiative_by_id(&room_id_metadata)
                    .await
                else {
                    content.set(wrapper.info.description);
                    continue;
                };

                room_id.set(Some(room_id_metadata));

                content.set(response.info.description.clone());

                log::info!("{:#?}", response);
                wrapper.info = response.info.clone();

                initiative_wrapper.set(Some(wrapper.clone()));
            }

            if community_tracks.iter().any(|community| community.id == id) {
                can_vote.set(true);
            }
        }
    });

    let handle_vote = move |is_vote_aye: bool| {
        spawn(
            async move {
                tooltip.handle_tooltip(TooltipItem {
                    title: translate!(i18, "governance.tips.voting.title"),
                    body: translate!(i18, "governance.tips.voting.description"),
                    show: true,
                });

                let account_address = session
                    .get()
                    .ok_or(translate!(i18, "errors.wallet.account_address"))?
                    .address;

                let address =
                    sp_core::sr25519::Public::from_str(&account_address).map_err(|e| {
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
                            vote: Vote::Standard(if is_vote_aye {
                                VoteOf::Yes
                            } else {
                                VoteOf::No
                            }),
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

                notification.handle_success(&translate!(i18, "governance.tips.voted.description"));

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

    use_coroutine(move |_: UnboundedReceiver<()>| async move { on_handle_vote.send(()) });

    rsx! {
        div { class: "page--vote",
            div { class: "initiative__form",
                if let Some(initiative_wrapper) = &*initiative_wrapper.read() {
                    div { class: "form__wrapper form__wrapper--initiative",
                        h2 { class: "form__title",
                            "{initiative_wrapper.info.name}"
                        }
                        div { class: "details__metadata",
                            KeyValue {
                                class: "key-value",
                                text: format!("{}: ", translate!(i18, "governance.description.details.by")),
                                size: ElementSize::Medium,
                                variant: KeyValueVariant::Secondary,
                                body: rsx!(
                                    {
                                        let hex_string = hex::encode(&initiative_wrapper.ongoing.submission_deposit.who);
                                        format!("0x{}", hex_string)
                                    }
                                )
                            }
                        }
                        div { class: "steps__wrapper",
                            div { class: "row",
                                section { class: "details__voting",
                                    div { class: "vote-card",
                                        h4 { class: "vote-card__title",
                                            "Request"
                                        }
                                        button { class: "button--tertiary",
                                            onclick: move |_| show_requests.toggle(),
                                            Request {
                                                name: if show_requests() { "Hide all requests" } else { "See all requests" },
                                                details: initiative_wrapper.info.actions.iter().map(|item| {
                                                    match item {
                                                        ActionItem::AddMembers(action) => action.members.len(),
                                                        ActionItem::KusamaTreasury(action) => action.periods.len(),
                                                        ActionItem::VotingOpenGov(action) => action.proposals.len(),
                                                    }
                                                }).sum::<usize>().to_string(),
                                                size: ElementSize::Small
                                            }
                                        }
                                        if show_requests() {
                                            {
                                                initiative_wrapper.info.actions.iter().map(|request| {
                                                    rsx!(
                                                        div { class: "requests",
                                                            match request {
                                                                ActionItem::AddMembers(action) => {
                                                                    rsx!(
                                                                        Request {
                                                                            name: "Add Members",
                                                                            details: action.members.len().to_string()
                                                                        }
                                                                        ul { class: "requests",
                                                                            {
                                                                                action.members.iter().map(|member| {
                                                                                    rsx!(
                                                                                        li {
                                                                                            Request {
                                                                                                name: format!("{}...", member.account[..10].to_string()),
                                                                                            }
                                                                                        }
                                                                                    )
                                                                                })
                                                                            }
                                                                        }
                                                                    )
                                                                },
                                                                ActionItem::KusamaTreasury(action) => {
                                                                    rsx!(
                                                                        Request {
                                                                            name: "Kusama Treasury Request"
                                                                        }
                                                                        ul { class: "requests",
                                                                            {
                                                                                action.periods.iter().enumerate().map(|(index, period)| {
                                                                                    rsx!(
                                                                                        li {
                                                                                            Request {
                                                                                                name: format!("Periodo: #{}", index + 1),
                                                                                                details: format!("{} KSM", period.amount as f64 / 1_000_000_000_000.0 )
                                                                                            }
                                                                                        }
                                                                                    )
                                                                                })
                                                                            }
                                                                        }
                                                                    )
                                                                },
                                                                ActionItem::VotingOpenGov(action) => {
                                                                    rsx!(
                                                                        Request {
                                                                            name: "Voting Open Gov",
                                                                            details: action.proposals.len().to_string()
                                                                        }
                                                                        ul { class: "requests",
                                                                            {
                                                                                action.proposals.iter().map(|proposal| {
                                                                                    rsx!(
                                                                                        li {
                                                                                            match &proposal.vote {
                                                                                                VoteType::Standard(vote) => {
                                                                                                    let conviction = match vote.conviction {
                                                                                                        ConvictionVote::None => translate!(i18, "initiative.steps.actions.voting_open_gov.standard.conviction.none"),
                                                                                                        ConvictionVote::Locked1x => translate!(i18, "initiative.steps.actions.voting_open_gov.standard.conviction.locked_1"),
                                                                                                        ConvictionVote::Locked2x => translate!(i18, "initiative.steps.actions.voting_open_gov.standard.conviction.locked_2"),
                                                                                                        ConvictionVote::Locked3x => translate!(i18, "initiative.steps.actions.voting_open_gov.standard.conviction.locked_3"),
                                                                                                        ConvictionVote::Locked4x => translate!(i18, "initiative.steps.actions.voting_open_gov.standard.conviction.locked_4"),
                                                                                                        ConvictionVote::Locked5x => translate!(i18, "initiative.steps.actions.voting_open_gov.standard.conviction.locked_5"),
                                                                                                        ConvictionVote::Locked6x => translate!(i18, "initiative.steps.actions.voting_open_gov.standard.conviction.locked_6"),
                                                                                                    };
                                                                                                    rsx!(
                                                                                                        Request {
                                                                                                            name: format!("{} - {}", translate!(i18, "initiative.steps.actions.voting_open_gov.standard.title"), proposal.poll_index),
                                                                                                            details: format!("{} - {} KSM", conviction, vote.balance as f64 / 1_000_000_000_000.0 ),
                                                                                                        }
                                                                                                    )
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    )
                                                                                })
                                                                            }
                                                                        }
                                                                    )
                                                                },
                                                            }
                                                        }
                                                    )
                                                })
                                            }
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
                                                        div {
                                                            class: "statistics__bar statistics__bar--remaign",
                                                                span {
                                                                    class: "statistics__bar__content statistics__bar__content--consumed",
                                                                    style: format!("width: {}%", consumed_percent),
                                                                }
                                                                span {
                                                                    class: "statistics__bar__content statistics__bar__content--right",
                                                                    style: format!("width: {}%", 100.0 - consumed_percent),
                                                                    p { class: "votes-counter__title",
                                                                        if blocks_to_days(decision - consumed) == 0 {
                                                                            "{blocks_to_days(decision - consumed) + 1}"
                                                                        } else {
                                                                            "{blocks_to_days(decision - consumed)}"
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            div {
                                                                class: "statistics__bar__percent",
                                                                p { class: "votes-counter__percent",
                                                                "Decision"
                                                            }
                                                            p { class: "votes-counter__percent",
                                                                match blocks_to_times(decision) {
                                                                    Times::Minutes(time) => {format!("{} Minutes", time)},
                                                                    Times::Hours(time) => {format!("{} Hours", time)},
                                                                    Times::Days(time) => {format!("{} Days", time)},
                                                                }
                                                            }
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
                                                    {translate!(i18, "governance.description.voting.title")}
                                                }
                                                Button {
                                                    text: if show_vote() { "Hide vote" } else { "Vote" },
                                                    size: ElementSize::Small,
                                                    variant: Variant::Secondary,
                                                    on_click: move |_| {
                                                        show_vote.toggle();
                                                    },
                                                    status: None,
                                                }
                                            }
                                            if show_vote() {
                                                div { class: "note",
                                                    "Explain that this is a dynamic voting, and thresholds might change."
                                                }
                                            }
                                            if show_vote() {
                                            // if can_vote() {
                                                div { class: "row",
                                                    Button {
                                                        class: "vote-cta",
                                                        text: translate!(i18, "governance.description.voting.cta.for"),
                                                        size: ElementSize::Medium,
                                                        variant: Variant::Secondary,
                                                        on_click: move |_| {
                                                            handle_vote(true)
                                                        },
                                                        status: None,
                                                        left_icon: rsx!(
                                                            Icon {
                                                                icon: CircleCheck,
                                                                height: 16,
                                                                width: 16,
                                                                stroke_width: 2,
                                                                stroke: "#56C95F"
                                                            }
                                                        )
                                                    }
                                                    Button {
                                                        class: "vote-cta",
                                                        text: translate!(i18, "governance.description.voting.cta.against"),
                                                        size: ElementSize::Medium,
                                                        variant: Variant::Secondary,
                                                        on_click: move |_| {
                                                            handle_vote(false)
                                                        },
                                                        status: None,
                                                        left_icon: rsx!(
                                                            Icon {
                                                                icon: StopSign,
                                                                height: 16,
                                                                width: 16,
                                                                stroke_width: 2,
                                                                stroke: "#f44336bd"
                                                            }
                                                        )
                                                    }
                                                }
                                            // }
                                            }
                                            div {
                                                div {
                                                    class: "statistics__bar statistics__bar--vote",
                                                    span {
                                                        class: "statistics__bar__content statistics__bar__content--aye",
                                                        style: format!("width: {}%", votes_statistics().percent_aye()),
                                                        p { class: "votes-counter__title",
                                                            {translate!(i18, "governance.description.voting.for")}
                                                        }
                                                    }
                                                    span {
                                                        class: "statistics__bar__content statistics__bar__content--nay statistics__bar__content--right",
                                                        style: format!("width: {}%", votes_statistics().percent_nay()),
                                                        p { class: "votes-counter__title",
                                                            {translate!(i18, "governance.description.voting.against")}
                                                        }
                                                    }
                                                }
                                                div {
                                                    class: "statistics__bar__percent",
                                                    p { class: "votes-counter__percent",
                                                        {format!("{:.1}%", votes_statistics().percent_aye())}
                                                    }
                                                    p { class: "votes-counter__percent",
                                                        {format!("{:.1}%", votes_statistics().percent_nay())}
                                                    }
                                                }
                                            }
                                            if show_vote() {
                                                div { class: "note",
                                                    KeyValue {
                                                        class: "key-value--row",
                                                        text: "Threshold",
                                                        size: ElementSize::Medium,
                                                        body: rsx!(
                                                            {
                                                                let threshold = match &*track_info.read() {
                                                                    Some(info) => {
                                                                        if let Some(deciding) = &initiative_wrapper.ongoing.deciding {
                                                                            if  current_block() > 0 {
                                                                                let consumed = current_block() - deciding.since;
                                                                                let progress = consumed as f64 / 36000.0;

                                                                                match info.min_approval {
                                                                                    Curve::LinearDecreasing { ceil, floor, length } => {
                                                                                        let length = length as f64 / 10_000_000.0;
                                                                                        let ceil = ceil as f64 / 10_000_000.0;
                                                                                        let floor = floor as f64 / 10_000_000.0;

                                                                                        let progress = progress / (length / 100.0);
                                                                                        ceil - progress * (ceil - floor)
                                                                                    },
                                                                                    Curve::SteppedDecreasing { begin: _, end: _, step: _, period: _ } => 100.0,
                                                                                    Curve::Reciprocal { factor: _, x_offset: _, y_offset: _ } => 100.0,
                                                                                }
                                                                            } else {
                                                                                0.0
                                                                            }
                                                                        } else {
                                                                            0.0
                                                                        }
                                                                    },
                                                                    None => 0.0
                                                                };
                                                                format!("{:.1}%", threshold)
                                                            }
                                                        )
                                                    }
                                                    KeyValue {
                                                        class: "key-value--row",
                                                        text: "Current approval",
                                                        size: ElementSize::Medium,
                                                        body: rsx!(
                                                            {
                                                                format!("{:.1}%", votes_statistics().percent_aye())
                                                            }
                                                        )
                                                    }
                                                }
                                            }
                                            if show_vote() {
                                                div {
                                                    div { class: "votes-counter votes-counter--for",
                                                        Icon {
                                                            icon: CircleCheck,
                                                            height: 16,
                                                            width: 16,
                                                            stroke_width: 2,
                                                            stroke: "#56C95F"
                                                        }
                                                        p { class: "votes-counter__total",
                                                            "{votes_statistics().aye} " {translate!(i18, "governance.description.voting.votes")}
                                                        }
                                                    }

                                                    div { class: "votes-counter votes-counter--against",
                                                        Icon {
                                                            icon: StopSign,
                                                            height: 16,
                                                            width: 16,
                                                            stroke_width: 2,
                                                            stroke: "#f44336bd"
                                                        }
                                                        p { class: "votes-counter__total",
                                                            "{votes_statistics().nay} " {translate!(i18, "governance.description.voting.votes")}
                                                        }
                                                    }
                                                }

                                                div {
                                                    {
                                                        let consumed_percent =  100.0 / members() as f64 * votes_statistics().total() as f64;

                                                        rsx!(
                                                            div {
                                                                class: "statistics__bar statistics__bar--remaign",
                                                                    span {
                                                                        class: "statistics__bar__content statistics__bar__content--consumed",
                                                                        style: format!("width: {}%", consumed_percent),
                                                                        p { class: "votes-counter__title",
                                                                            "Participation"
                                                                        }
                                                                    }
                                                                    span {
                                                                        class: "statistics__bar__content statistics__bar__content--right",
                                                                        style: format!("width: {}%", 100.0 - consumed_percent),
                                                                    }
                                                                }
                                                                div {
                                                                    class: "statistics__bar__percent",
                                                                    p { class: "votes-counter__percent",
                                                                    "{votes_statistics().total()}"
                                                                }
                                                                p { class: "votes-counter__percent",
                                                                    "{members()}"
                                                                }
                                                            }
                                                        )
                                                    }
                                                }
                                                div { class: "note",
                                                    KeyValue {
                                                        class: "key-value--row",
                                                        text: "Paricipation threshold",
                                                        size: ElementSize::Medium,
                                                        body: rsx!(
                                                            {
                                                                let threshold = match &*track_info.read() {
                                                                    Some(info) => {
                                                                        if let Some(deciding) = &initiative_wrapper.ongoing.deciding {
                                                                            if  current_block() > 0 {
                                                                                let consumed = current_block() - deciding.since;
                                                                                let progress = consumed as f64 / 36000.0;

                                                                                match info.min_support {
                                                                                    Curve::LinearDecreasing { ceil, floor, length } => {
                                                                                        let length = length as f64 / 10_000_000.0;
                                                                                        let ceil = ceil as f64 / 10_000_000.0;
                                                                                        let floor = floor as f64 / 10_000_000.0;

                                                                                        let progress = progress / (length / 100.0);
                                                                                        ceil - progress * (ceil - floor)
                                                                                    },
                                                                                    Curve::SteppedDecreasing { begin: _, end: _, step: _, period: _ } => 100.0,
                                                                                    Curve::Reciprocal { factor: _, x_offset: _, y_offset: _ } => 100.0,
                                                                                }
                                                                            } else {
                                                                                100.0
                                                                            }
                                                                        } else {
                                                                            100.0
                                                                        }
                                                                    },
                                                                    None => 100.0
                                                                };
                                                                format!("{:.1}%", threshold)
                                                            }
                                                        )
                                                    }
                                                    KeyValue {
                                                        class: "key-value--row",
                                                        text: "Current support",
                                                        size: ElementSize::Medium,
                                                        body: rsx!(
                                                            {
                                                                let consumed_percent =  100.0 / members() as f64 * votes_statistics().total() as f64;
                                                                format!("{:.1}%", consumed_percent)
                                                            }
                                                        )
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            section { class: "details__proposal",
                                div { class: "details__subtitle",
                                    "Content"
                                }
                                div { class: "details__tags",
                                    div { class: "card__tags",
                                        for tag in initiative_wrapper.clone().info.tags {
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

                                div { class: "details__description markdown-preview",
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
