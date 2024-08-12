use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::atoms::{
        combo_input::{ComboInputOption, ComboInputValue},
        dropdown::{DropdownItem, ElementSize},
        icon_button::Variant,
        AddPlus, ComboInput, Icon, IconButton, Input, MinusCircle, RadioButton,
    },
    hooks::use_initiative::{
        use_initiative, ActionItem, ConvictionVote, SplitAbstainVote, SplitVote, StandardVote,
        VoteType, VotingOpenGov, VotingOpenGovAction,
    },
};

#[derive(PartialEq, Props, Clone)]
pub struct VotingProps {
    index: usize,
    meta: VotingOpenGovAction,
}

const KUSAMA_PRECISION_DECIMALS: u64 = 1_000_000_000_000;

pub fn VotingAction(props: VotingProps) -> Element {
    let i18 = use_i18();
    let mut initiative = use_initiative();

    rsx!(
        ul { class: "form__inputs form__inputs--combo",
            {
                props.meta.proposals.iter().enumerate().map(|(index_meta, proposal)| {
                    rsx!(
                        li {
                            div { class: "form__inputs__wrapper",
                                div {
                                    ComboInput {
                                        size: ElementSize::Small,
                                        value: ComboInputValue {
                                            option: ComboInputOption::Dropdown(DropdownItem {
                                                key: "Standard".to_string(),
                                                value: "Standard".to_string(),
                                            }),
                                            input: proposal.poll_index.to_string()
                                        },
                                        placeholder: translate!(i18, "initiative.steps.actions.voting_open_gov.poll_index"),
                                        options: vec![
                                            DropdownItem {
                                                key: "Standard".to_string(),
                                                value: translate!(i18, "initiative.steps.actions.voting_open_gov.standard.title"),
                                            },
                                            DropdownItem {
                                                key: "Split".to_string(),
                                                value: translate!(i18, "initiative.steps.actions.voting_open_gov.split.title"),
                                            },
                                            DropdownItem {
                                                key: "SplitAbstain".to_string(),
                                                value: translate!(i18, "initiative.steps.actions.voting_open_gov.split_abstain.title"),
                                            }
                                        ],
                                        on_change: move |event: ComboInputValue| {
                                            if let ActionItem::VotingOpenGov(ref mut meta) = initiative.get_action(props.index) {
                                                let vote = match event.option {
                                                    ComboInputOption::Dropdown(value) => {
                                                        if value.key.as_str() == meta.proposals[index_meta].vote.key_string() {
                                                            meta.proposals[index_meta].vote.clone()
                                                        } else {
                                                            match value.key.as_str() {
                                                                "Standard" => VoteType::Standard(StandardVote::default()),
                                                                "Split" => VoteType::Split(SplitVote::default()),
                                                                "SplitAbstain" => VoteType::SplitAbstain(SplitAbstainVote::default()),
                                                                _ => VoteType::Standard(StandardVote::default())
                                                            }
                                                        }

                                                    },
                                                    _ => todo!()
                                                };

                                                let poll_index: u64 = event.input.parse().unwrap_or(0);
                                                meta.proposals[index_meta] = VotingOpenGov {
                                                    poll_index,
                                                    vote: vote
                                                };
                                                initiative.update_action(props.index, ActionItem::VotingOpenGov(meta.clone()));
                                            }
                                        }
                                    }
                                }
                                div {
                                    match &proposal.vote {
                                        VoteType::Standard(vote) => {
                                            let vote_a = vote.clone();
                                            let mut vote_b = vote.clone();
                                            let mut vote_c = vote.clone();

                                            rsx!(
                                                div {
                                                    ComboInput {
                                                        size: ElementSize::Small,
                                                        value: ComboInputValue {
                                                            option: ComboInputOption::Dropdown( DropdownItem {
                                                                key: "None".to_string(),
                                                                value: "None".to_string(),
                                                            }),
                                                            input: (vote.balance / KUSAMA_PRECISION_DECIMALS).to_string()
                                                        },
                                                        placeholder: translate!(i18, "initiative.steps.actions.voting_open_gov.standard.balance"),
                                                        options: vec![
                                                            DropdownItem {
                                                                key: "None".to_string(),
                                                                value: translate!(i18, "initiative.steps.actions.voting_open_gov.standard.conviction.none"),
                                                            },
                                                            DropdownItem {
                                                                key: "Locked1x".to_string(),
                                                                value: translate!(i18, "initiative.steps.actions.voting_open_gov.standard.conviction.locked_1"),
                                                            },
                                                            DropdownItem {
                                                                key: "Locked2x".to_string(),
                                                                value: translate!(i18, "initiative.steps.actions.voting_open_gov.standard.conviction.locked_2"),
                                                            },
                                                            DropdownItem {
                                                                key: "Locked3x".to_string(),
                                                                value: translate!(i18, "initiative.steps.actions.voting_open_gov.standard.conviction.locked_3"),
                                                            },
                                                            DropdownItem {
                                                                key: "Locked4x".to_string(),
                                                                value: translate!(i18, "initiative.steps.actions.voting_open_gov.standard.conviction.locked_4"),
                                                            },
                                                            DropdownItem {
                                                                key: "Locked5x".to_string(),
                                                                value: translate!(i18, "initiative.steps.actions.voting_open_gov.standard.conviction.locked_5"),
                                                            },
                                                            DropdownItem {
                                                                key: "Locked6x".to_string(),
                                                                value: translate!(i18, "initiative.steps.actions.voting_open_gov.standard.conviction.locked_6"),
                                                            }
                                                        ],
                                                        on_change: move |event: ComboInputValue| {
                                                            let conviction = match event.option {
                                                                ComboInputOption::Dropdown(value) => {
                                                                    match value.key.as_str() {
                                                                        "None" => ConvictionVote::None,
                                                                        "Locked1x" => ConvictionVote::Locked1x,
                                                                        "Locked2x" => ConvictionVote::Locked2x,
                                                                        "Locked3x" => ConvictionVote::Locked3x,
                                                                        "Locked4x" => ConvictionVote::Locked4x,
                                                                        "Locked5x" => ConvictionVote::Locked5x,
                                                                        "Locked6x" => ConvictionVote::Locked6x,
                                                                        _ => todo!()
                                                                    }
                                                                },
                                                                _ => todo!()
                                                            };
                                                            if let ActionItem::VotingOpenGov(ref mut meta) = initiative.get_action(props.index) {
                                                                let amount = event.input.parse::<f64>().unwrap_or(0.0);
                                                                let scaled_amount = amount * KUSAMA_PRECISION_DECIMALS as f64;

                                                                meta.proposals[index_meta].vote = VoteType::Standard(
                                                                    StandardVote { aye: vote_a.aye, conviction, balance: scaled_amount as u64 }
                                                                );
                                                                initiative.update_action(props.index, ActionItem::VotingOpenGov(meta.clone()));
                                                            }
                                                        }
                                                    }
                                                    div { class: "form__inputs__container__cta",
                                                        RadioButton {
                                                            title: translate!(i18, "initiative.steps.actions.voting_open_gov.standard.aye"),
                                                            name: "Aye",
                                                            checked: vote.aye.clone(),
                                                            on_change: move |_| {
                                                                if let ActionItem::VotingOpenGov(ref mut meta) = initiative.get_action(props.index) {
                                                                    vote_b.aye = true;
                                                                    meta.proposals[index_meta].vote = VoteType::Standard(vote_b.clone());
                                                                    initiative.update_action(props.index, ActionItem::VotingOpenGov(meta.clone()));
                                                                }
                                                            }
                                                        }
                                                        RadioButton {
                                                            title: translate!(i18, "initiative.steps.actions.voting_open_gov.standard.nay"),
                                                            name: "Nay",
                                                            checked: !vote.aye.clone(),
                                                            on_change: move |_| {
                                                                if let ActionItem::VotingOpenGov(ref mut meta) = initiative.get_action(props.index) {
                                                                    vote_c.aye = false;
                                                                    meta.proposals[index_meta].vote = VoteType::Standard(vote_c.clone());
                                                                    initiative.update_action(props.index, ActionItem::VotingOpenGov(meta.clone()));
                                                                }
                                                            },
                                                        }
                                                    }
                                                }
                                            )
                                        },
                                        VoteType::Split(ref vote) => {
                                            let mut vote_a = vote.clone();
                                            let mut vote_b = vote.clone();

                                            rsx!(
                                                div { class: "form__inputs__container",
                                                    Input {
                                                        message: vote.aye / KUSAMA_PRECISION_DECIMALS,
                                                        size: ElementSize::Small,
                                                        placeholder: translate!(i18, "initiative.steps.actions.voting_open_gov.split.aye"),
                                                        error: None,
                                                        on_input: move |event: Event<FormData>| {
                                                            if let ActionItem::VotingOpenGov(ref mut meta) = initiative.get_action(props.index) {
                                                                // Scale amount
                                                                let amount = event.value().parse::<f64>().unwrap_or(0.0);
                                                                let scaled_amount = amount * KUSAMA_PRECISION_DECIMALS as f64;

                                                                vote_a.aye = scaled_amount as u64;
                                                                meta.proposals[index_meta].vote = VoteType::Split(vote_a.clone());
                                                                initiative.update_action(props.index, ActionItem::VotingOpenGov(meta.clone()));
                                                            }
                                                        },
                                                        on_keypress: move |_| {},
                                                        on_click: move |_| {},
                                                    }
                                                    Input {
                                                        message: vote.nay / KUSAMA_PRECISION_DECIMALS,
                                                        size: ElementSize::Small,
                                                        placeholder: translate!(i18, "initiative.steps.actions.voting_open_gov.split.nay"),
                                                        error: None,
                                                        on_input: move |event: Event<FormData>| {
                                                            if let ActionItem::VotingOpenGov(ref mut meta) = initiative.get_action(props.index) {
                                                                // Scale amount
                                                                let amount = event.value().parse::<f64>().unwrap_or(0.0);
                                                                let scaled_amount = amount * KUSAMA_PRECISION_DECIMALS as f64;

                                                                vote_b.nay = scaled_amount as u64;
                                                                meta.proposals[index_meta].vote = VoteType::Split(vote_b.clone());
                                                                initiative.update_action(props.index, ActionItem::VotingOpenGov(meta.clone()));
                                                            }
                                                        },
                                                        on_keypress: move |_| {},
                                                        on_click: move |_| {},
                                                    }
                                                }
                                            )
                                        },
                                        VoteType::SplitAbstain(vote) => {
                                            let mut vote_a = vote.clone();
                                            let mut vote_b = vote.clone();
                                            let mut vote_c = vote.clone();

                                            rsx!(
                                                div { class: "form__inputs__container",
                                                    Input {
                                                        message: vote.aye / KUSAMA_PRECISION_DECIMALS,
                                                        size: ElementSize::Small,
                                                        placeholder: translate!(i18, "initiative.steps.actions.voting_open_gov.split_abstain.aye"),
                                                        error: None,
                                                        on_input: move |event: Event<FormData>| {
                                                            if let ActionItem::VotingOpenGov(ref mut meta) = initiative.get_action(props.index) {
                                                                // Scale amount
                                                                let amount = event.value().parse::<f64>().unwrap_or(0.0);
                                                                let scaled_amount = amount * KUSAMA_PRECISION_DECIMALS as f64;

                                                                vote_a.aye = scaled_amount as u64;
                                                                meta.proposals[index_meta].vote = VoteType::SplitAbstain(vote_a.clone());
                                                                initiative.update_action(props.index, ActionItem::VotingOpenGov(meta.clone()));
                                                            }
                                                        },
                                                        on_keypress: move |_| {},
                                                        on_click: move |_| {},
                                                    }
                                                    Input {
                                                        message: vote.nay / KUSAMA_PRECISION_DECIMALS,
                                                        size: ElementSize::Small,
                                                        placeholder: translate!(i18, "initiative.steps.actions.voting_open_gov.split_abstain.nay"),
                                                        error: None,
                                                        on_input: move |event: Event<FormData>| {
                                                            if let ActionItem::VotingOpenGov(ref mut meta) = initiative.get_action(props.index) {
                                                                // Scale amount
                                                                let amount = event.value().parse::<f64>().unwrap_or(0.0);
                                                                let scaled_amount = amount * KUSAMA_PRECISION_DECIMALS as f64;

                                                                vote_b.nay = scaled_amount as u64;
                                                                meta.proposals[index_meta].vote = VoteType::SplitAbstain(vote_b.clone());
                                                                initiative.update_action(props.index, ActionItem::VotingOpenGov(meta.clone()));
                                                            }
                                                        },
                                                        on_keypress: move |_| {},
                                                        on_click: move |_| {},
                                                    }
                                                    Input {
                                                        message: vote.abstain / KUSAMA_PRECISION_DECIMALS,
                                                        size: ElementSize::Small,
                                                        placeholder: translate!(i18, "initiative.steps.actions.voting_open_gov.split_abstain.abstain"),
                                                        error: None,
                                                        on_input: move |event: Event<FormData>| {
                                                            if let ActionItem::VotingOpenGov(ref mut meta) = initiative.get_action(props.index) {
                                                                // Scale amount
                                                                let amount = event.value().parse::<f64>().unwrap_or(0.0);
                                                                let scaled_amount = amount * KUSAMA_PRECISION_DECIMALS as f64;

                                                                vote_c.abstain = scaled_amount as u64;
                                                                meta.proposals[index_meta].vote = VoteType::SplitAbstain(vote_c.clone());
                                                                initiative.update_action(props.index, ActionItem::VotingOpenGov(meta.clone()));
                                                            }
                                                        },
                                                        on_keypress: move |_| {},
                                                        on_click: move |_| {},
                                                    }
                                                }
                                            )
                                        },
                                    }
                                }
                            }

                            IconButton {
                                variant: Variant::Round,
                                size: ElementSize::Small,
                                class: "button--avatar",
                                body: rsx!(
                                    Icon {
                                        icon: MinusCircle,
                                        height: 24,
                                        width: 24,
                                        fill: "var(--state-primary-active)"
                                    }
                                ),
                                on_click: move |_| {
                                    if let ActionItem::VotingOpenGov(ref mut meta) = initiative.get_action(props.index) {
                                        meta.proposals.remove(index_meta);
                                        initiative.update_action(props.index, ActionItem::VotingOpenGov(meta.clone()));
                                    }
                                }
                            }
                        }
                    )
                })
            }
            IconButton {
                variant: Variant::Round,
                size: ElementSize::Small,
                class: "button--avatar",
                body: rsx!(
                    Icon {
                        icon: AddPlus,
                        height: 24,
                        width: 24,
                        fill: "var(--state-primary-active)"
                    }
                ),
                on_click: move |_| {
                    if let ActionItem::VotingOpenGov(ref mut meta) = initiative.get_action(props.index) {
                        meta.add_proposal(VotingOpenGov::default());
                        initiative.update_action(props.index, ActionItem::VotingOpenGov(meta.clone()));
                    }
                }
            }
        }
    )
}