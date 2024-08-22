use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use crate::{
    components::atoms::ActionRequest,
    hooks::use_initiative::{
        ActionItem, AddMembersAction, ConvictionVote, KusamaTreasuryAction, VoteType,
        VotingOpenGovAction,
    },
};
#[derive(PartialEq, Props, Clone)]
pub struct ActionRequestListProps {
    actions: Vec<ActionItem>,
}
pub fn ActionRequestList(props: ActionRequestListProps) -> Element {
    let i18 = use_i18();
    let render_add_members = |action: &AddMembersAction| {
        rsx!(
            ActionRequest { name: "Add Members", details: action.members.len().to_string() }
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
            ActionRequest { name: "Kusama Treasury Request" }
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
            ActionRequest { name: "Voting Open Gov", details: action.proposals.len().to_string() }
            ul { class: "requests",
                for proposal in action.proposals.iter() {
                    li {
                        match & proposal.vote { VoteType::Standard(vote) => { let
                        conviction = match vote.conviction { ConvictionVote::None => translate!(i18,
                        "initiative.steps.actions.voting_open_gov.standard.conviction.none"),
                        ConvictionVote::Locked1x => translate!(i18,
                        "initiative.steps.actions.voting_open_gov.standard.conviction.locked_1"),
                        ConvictionVote::Locked2x => translate!(i18,
                        "initiative.steps.actions.voting_open_gov.standard.conviction.locked_2"),
                        ConvictionVote::Locked3x => translate!(i18,
                        "initiative.steps.actions.voting_open_gov.standard.conviction.locked_3"),
                        ConvictionVote::Locked4x => translate!(i18,
                        "initiative.steps.actions.voting_open_gov.standard.conviction.locked_4"),
                        ConvictionVote::Locked5x => translate!(i18,
                        "initiative.steps.actions.voting_open_gov.standard.conviction.locked_5"),
                        ConvictionVote::Locked6x => translate!(i18,
                        "initiative.steps.actions.voting_open_gov.standard.conviction.locked_6"), };
                        rsx!(ActionRequest { name : format!("{} - {}", translate!(i18,
                        "initiative.steps.actions.voting_open_gov.standard.title"), proposal
                        .poll_index), details : format!("{} - {} KSM", conviction, vote.balance as
                        f64 / 1_000_000_000_000.0), }) } }
                    }
                }
            }
        )
    };
    rsx!(
        for request in props.actions.iter() {
            div { class: "requests",
                match request {
                ActionItem::AddMembers(action) => render_add_members(& action),
                ActionItem::KusamaTreasury(action) => render_kusama_treasury(& action),
                ActionItem::VotingOpenGov(action) => render_voting_open_gov(& action) }
            }
        }
    )
}
