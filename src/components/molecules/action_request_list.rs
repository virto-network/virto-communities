use crate::{
    components::atoms::ActionRequest,
    hooks::use_initiative::{
        ActionItem, AddMembersAction, CommunityTransferAction, ConvictionVote,
        KusamaTreasuryAction, VoteType, VotingOpenGovAction,
    },
};
use dioxus::prelude::*;
use dioxus_i18n::t;
#[derive(PartialEq, Props, Clone)]
pub struct ActionRequestListProps {
    actions: Vec<ActionItem>,
}
pub fn ActionRequestList(props: ActionRequestListProps) -> Element {
    let render_add_members = |action: &AddMembersAction| {
        rsx!(
            ActionRequest {
                name: t!("initiative-steps-actions-add_members-title"),
                details: action.members.len().to_string()
            }
            ul { class: "requests",
                for member in action.members.iter() {
                    li {
                        ActionRequest { name: format!("{}...", member.account[..10].to_string()) }
                    }
                }
            }
        )
    };
    let render_kusama_treasury = |action: &KusamaTreasuryAction| {
        rsx!(
            ActionRequest { name: t!("initiative-steps-actions-kusama_treasury-title") }
            ul { class: "requests",
                for (index , period) in action.periods.iter().enumerate() {
                    li {
                        ActionRequest {
                            name: format!("Periodo: #{}", index + 1),
                            details: format!("{} KSM", period.amount as f64 / 1_000_000_000_000.0)
                        }
                    }
                }
            }
        )
    };
    let render_voting_open_gov = |action: &VotingOpenGovAction| {
        rsx!(
            ActionRequest {
                name: t!("initiative-steps-actions-voting_open_gov-title"),
                details: action.proposals.len().to_string()
            }
            ul { class: "requests",
                for proposal in action.proposals.iter() {
                    li {
                        match & proposal.vote { VoteType::Standard(vote) => { let
                        conviction = match vote.conviction { ConvictionVote::None => t!(
                        "initiative.steps.actions.voting_open_gov.standard.conviction.none"),
                        ConvictionVote::Locked1x => t!(
                        "initiative.steps.actions.voting_open_gov.standard.conviction.locked_1"),
                        ConvictionVote::Locked2x => t!(
                        "initiative.steps.actions.voting_open_gov.standard.conviction.locked_2"),
                        ConvictionVote::Locked3x => t!(
                        "initiative.steps.actions.voting_open_gov.standard.conviction.locked_3"),
                        ConvictionVote::Locked4x => t!(
                        "initiative.steps.actions.voting_open_gov.standard.conviction.locked_4"),
                        ConvictionVote::Locked5x => t!(
                        "initiative.steps.actions.voting_open_gov.standard.conviction.locked_5"),
                        ConvictionVote::Locked6x => t!(
                        "initiative.steps.actions.voting_open_gov.standard.conviction.locked_6"), };
                        rsx!(ActionRequest { name : format!("{} - {}", t!(
                        "initiative.steps.actions.voting_open_gov.standard.title"), proposal
                        .poll_index), details : format!("{} - {} KSM", conviction, vote.balance as
                        f64 / 1_000_000_000_000.0), }) } }
                    }
                }
            }
        )
    };
    let render_community_transfer = |action: &CommunityTransferAction| {
        rsx!(
            ActionRequest { name: t!("initiative-steps-actions-community_transfer-title") }
            ul { class: "requests",
                for transfer in action.transfers.iter() {
                    li {
                        ActionRequest {
                            name: format!("{}", transfer.account),
                            details: format!("{} KSM", transfer.value as f64 / 1_000_000_000_000.0)
                        }
                    }
                }
            }
        )
    };
    rsx!(
        for request in props.actions.iter() {
            div { class: "requests",
                match request {
                ActionItem::AddMembers(action) => render_add_members(action),
                ActionItem::KusamaTreasury(action) => render_kusama_treasury(action),
                ActionItem::VotingOpenGov(action) => render_voting_open_gov(action) ,
                ActionItem::CommunityTransfer(action) => render_community_transfer(action) }
            }
        }
    )
}
