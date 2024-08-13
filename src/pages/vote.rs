use std::{collections::HashMap, str::FromStr};

use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use futures_util::{StreamExt, TryFutureExt};

use crate::{
    components::atoms::{
        button::Variant, dropdown::ElementSize, key_value::Variant as KeyValueVariant, Badge,
        Button, CircleCheck, Icon, KeyValue, Request, StopSign,
    },
    hooks::{
        use_accounts::use_accounts,
        use_initiative::{
            use_initiative, ActionItem, AddMembersAction, ConvictionVote, InitiativeHistory,
            InitiativeInfoContent, InitiativeVoteContent, InitiativeVoteData, KusamaTreasury,
            KusamaTreasuryAction, MemberItem, StandardVote, Vote, VoteOf, VoteType, VotingOpenGov,
            VotingOpenGovAction,
        },
        use_notification::use_notification,
        use_our_navigator::use_our_navigator,
        use_session::use_session,
        use_spaces_client::use_spaces_client,
        use_tooltip::use_tooltip,
    },
    pages::{initiatives::InitiativeWrapper, onboarding::convert_to_jsvalue},
    services::kreivo::{
        community_memberships::{get_communities_by_member, get_membership_id},
        community_referenda::{metadata_of, referendum_info_for},
        preimage::{preimage_for, request_status_for},
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

    fn add_aye(&mut self) {
        self.aye = self.aye + 1
    }

    fn add_nay(&mut self) {
        self.nay = self.nay + 1
    }

    fn set_aye(&mut self, aye: u64) {
        self.aye = aye
    }

    fn set_nay(&mut self, nay: u64) {
        self.nay = nay
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

fn filter_latest_votes(votes: Vec<InitiativeVoteContent>) -> Vec<InitiativeVoteContent> {
    let mut latest_votes: HashMap<String, InitiativeVoteContent> = HashMap::new();

    for vote in votes.iter().rev() {
        latest_votes.insert(vote.user.clone(), vote.clone());
    }

    latest_votes.into_values().collect()
}

#[component]
pub fn Vote(id: u16, initiativeid: u16) -> Element {
    let i18 = use_i18();
    let mut initiative = use_initiative();
    let mut session = use_session();
    let spaces_client = use_spaces_client();
    let mut nav = use_our_navigator();

    let mut notification = use_notification();
    let mut tooltip = use_tooltip();
    let accounts = use_accounts();

    let mut votes_statistics = use_signal(|| VoteDigest::default());
    let mut content = use_signal(|| String::new());
    let mut can_vote = use_signal(|| false);

    let mut initiative_wrapper = consume_context::<Signal<Option<InitiativeWrapper>>>();

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

            if community_tracks.iter().any(|community| community.id == id) {
                can_vote.set(true);
            }

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

            if let Some(mut wrapper) = initiative_wrapper() {
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

                content.set(response.info.description.clone());

                log::info!("{:?}", response);
                wrapper.info = response.info.clone();

                initiative_wrapper.set(Some(wrapper.clone()));
            }
        }
    });

    let mut handle_vote = move |is_vote_aye: bool| {
        spawn(
            async move {
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

                let response = spaces_client
                    .get()
                    .vote_initiative(InitiativeVoteData {
                        user: account_address,
                        room: String::from("!aOgBsDPlVOIDTisUsJ:matrix.org"),
                        vote: Vote::Standard(if is_vote_aye { VoteOf::Yes } else { VoteOf::No }),
                    })
                    .await;

                topup_then_initiative_vote(membership_id, initiativeid, is_vote_aye).await;

                on_handle_vote.send(());
                let path = format!("/dao/{id}/initiatives");
                nav.push(vec![], &path);

                Ok::<(), String>(())
            }
            .unwrap_or_else(move |e: String| {}),
        );
    };

    use_coroutine(move |_: UnboundedReceiver<()>| async move { on_handle_vote.send(()) });

    let requests = [
        ActionItem::AddMembers(AddMembersAction {
            members: vec![
                MemberItem {
                    medium: crate::hooks::use_initiative::MediumOptions::Wallet,
                    account: "5Gq2VNFP4yqK1bk5zt552hBxU68Q3ABewGN99zY7qGbpVTFc".to_string(),
                },
                MemberItem {
                    medium: crate::hooks::use_initiative::MediumOptions::Wallet,
                    account: "5Gq2VNFP4yqK1bk5zt552hBxU68Q3ABewGN99zY7qGbpVTFc".to_string(),
                },
            ],
        }),
        ActionItem::KusamaTreasury(KusamaTreasuryAction {
            periods: vec![
                KusamaTreasury {
                    date: "2024-12-10".to_string(),
                    amount: 1_100_000_000_000,
                },
                KusamaTreasury {
                    date: "2024-12-10".to_string(),
                    amount: 20_000_000_000_000,
                },
            ],
        }),
        ActionItem::VotingOpenGov(VotingOpenGovAction {
            proposals: vec![VotingOpenGov {
                poll_index: 400,
                vote: VoteType::Standard(StandardVote {
                    aye: true,
                    conviction: ConvictionVote::Locked1x,
                    balance: 20_000_000_000_000,
                }),
            }],
        }),
    ];

    let mut show_requests = use_signal(|| false);
    let mut show_vote = use_signal(|| false);

    rsx! {
        div { class: "page--initiative",
            div { class: "initiative__form",
                if let Some(ref initiative) = &*initiative_wrapper.read() {
                    div { class: "form__wrapper form__wrapper--initiative",
                        h2 { class: "form__title",
                            "{initiative.info.name}"
                        }
                        div { class: "details__metadata",
                            KeyValue {
                                class: "key-value",
                                text: format!("{}: ", translate!(i18, "governance.description.details.by")),
                                size: ElementSize::Medium,
                                variant: KeyValueVariant::Secondary,
                                body: rsx!(
                                    {
                                        let hex_string = hex::encode(&initiative.ongoing.submission_deposit.who);
                                        format!("0x{}", hex_string)
                                    }
                                )
                            }
                        }
                        div { class: "steps__wrapper",
                            div { class: "row",
                                div { class: "vote-card",
                                    h4 { class: "vote-card__title",
                                        "Request"
                                    }
                                    button { class: "button--tertiary",
                                        onclick: move |_| show_requests.toggle(),
                                        Request {
                                            name: if show_requests() { "Hide all requests" } else { "See all requests" },
                                            details: "9",
                                            size: ElementSize::Small
                                        }
                                    }
                                    if show_requests() {
                                        {
                                            requests.iter().map(|request| {
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
                                                                                            name: format!("Periodo: #{}", index),
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
                                                                                                        details: format!("{} - {}", conviction, vote.balance as f64 / 1_000_000_000_000.0 ),
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
                                div { class: "vote-card",
                                    KeyValue {
                                        class: "key-value--row",
                                        text: translate!(i18, "governance.description.details.status.title"),
                                        variant: KeyValueVariant::Secondary,
                                        body: {
                                            let status = ProposalStatus::VOTING;
                                            let (badge_title, badge_color) = match status {
                                                ProposalStatus::APPROVED => (translate!(i18, "governance.description.details.status.options.approved"), "badge--green-dark"),
                                                ProposalStatus::REJECTED => (translate!(i18, "governance.description.details.status.options.rejected"), "badge--red-dark"),
                                                ProposalStatus::VOTING => (translate!(i18, "governance.description.details.status.options.voting"), "badge--lavanda-dark"),
                                            };

                                            rsx!(
                                                Badge {
                                                    text: badge_title,
                                                    class: badge_color.to_string()
                                                }
                                            )
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
                                                div { class: "badge badge--note",
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
                                                    class: "statistics__bar",
                                                    span {
                                                        class: "statistics__bar__content statistics__bar__content--aye",
                                                        style: format!("width: {}%", votes_statistics().percent_aye()),
                                                        p { class: "votes-counter__title",
                                                            {translate!(i18, "governance.description.voting.for")}
                                                        }
                                                    }
                                                    span {
                                                        class: "statistics__bar__content statistics__bar__content--nay",
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

                                                div { class: "statistics__card",

                                                    KeyValue {
                                                        class: "key-value--row",
                                                        size: ElementSize::Small,
                                                        text: translate!(i18, "governance.description.voting.total.title"),
                                                        body: rsx!(
                                                            "{votes_statistics().total()} " {translate!(i18, "governance.description.voting.total.voters")}
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
                                        for tag in initiative.clone().info.tags {
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
            div { class: "form__cta form__cta--initiatives",
                p { class: "wip",
                    {translate!(i18, "initiative.disclaimer")}
                }
            }
        }
    }
}
