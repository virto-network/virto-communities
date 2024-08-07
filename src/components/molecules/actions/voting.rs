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
                            div {
                                style: "
                                    width: 100%;
                                ",
                                div {
                                    style: "
                                        width: 100%;
                                    ",
                                    ComboInput {
                                        size: ElementSize::Small,
                                        value: ComboInputValue {
                                            option: ComboInputOption::Dropdown(DropdownItem {
                                                key: "Standard".to_string(),
                                                value: "Standard".to_string(),
                                            }),
                                            input: proposal.poll_index.to_string()
                                        },
                                        placeholder: "Selecciona...",
                                        options: vec![
                                            DropdownItem {
                                                key: "Standard".to_string(),
                                                value: "Standard".to_string(),
                                            },
                                            DropdownItem {
                                                key: "Split".to_string(),
                                                value: "Split".to_string(),
                                            },
                                            DropdownItem {
                                                key: "SplitAbstain".to_string(),
                                                value: "SplitAbstain".to_string(),
                                            }
                                        ],
                                        on_change: move |event: ComboInputValue| {
                                            let vote = match event.option {
                                                ComboInputOption::Dropdown(value) => {
                                                    match value.key.as_str() {
                                                        "Standard" => VoteType::Standard(StandardVote::default()),
                                                        "Split" => VoteType::Split(SplitVote::default()),
                                                        "SplitAbstain" => VoteType::SplitAbstain(SplitAbstainVote::default()),
                                                        _ => todo!()
                                                    }
                                                },
                                                _ => todo!()
                                            };
                                            if let ActionItem::VotingOpenGov(ref mut meta) = initiative.get_action(props.index) {
                                                let poll_index: u64 = event.input.parse().unwrap_or(0);
                                                meta.proposals[index_meta] = VotingOpenGov {
                                                    poll_index,
                                                    vote
                                                };
                                                initiative.update_action(props.index, ActionItem::VotingOpenGov(meta.clone()));
                                            }
                                        }
                                    }
                                }
                                div {
                                    style: "
                                        width: 100%;
                                        margin-top: 4px;
                                    ",
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
                                                            input: vote.balance.to_string()
                                                        },
                                                        placeholder: "Selecciona...",
                                                        options: vec![
                                                            DropdownItem {
                                                                key: "None".to_string(),
                                                                value: "None".to_string(),
                                                            },
                                                            DropdownItem {
                                                                key: "Locked1x".to_string(),
                                                                value: "Locked x1".to_string(),
                                                            },
                                                            DropdownItem {
                                                                key: "Locked2x".to_string(),
                                                                value: "Locked x2".to_string(),
                                                            },
                                                            DropdownItem {
                                                                key: "Locked3x".to_string(),
                                                                value: "Locked x3".to_string(),
                                                            },
                                                            DropdownItem {
                                                                key: "Locked4x".to_string(),
                                                                value: "Locked x4".to_string(),
                                                            },
                                                            DropdownItem {
                                                                key: "Locked5x".to_string(),
                                                                value: "Locked x5".to_string(),
                                                            },
                                                            DropdownItem {
                                                                key: "Locked6x".to_string(),
                                                                value: "Locked x6".to_string(),
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
                                                                let balance: u64 = event.input.parse().unwrap_or(0);

                                                                meta.proposals[index_meta].vote = VoteType::Standard(
                                                                    StandardVote { aye: vote_a.aye, conviction, balance }
                                                                );
                                                                initiative.update_action(props.index, ActionItem::VotingOpenGov(meta.clone()));
                                                            }
                                                        }
                                                    }
                                                    div {
                                                        style: "
                                                            margin-top: 4px;
                                                            display: flex;
                                                            gap: 4px;
                                                            width: 100%;
                                                        ",
                                                        RadioButton {
                                                            title: "Vote Aye",
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
                                                            title: "Vote Nay",
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
                                                div {
                                                    style: "
                                                        display: flex;
                                                        gap: 4px;
                                                    ",
                                                    Input {
                                                        message: vote.aye / KUSAMA_PRECISION_DECIMALS,
                                                        size: ElementSize::Small,
                                                        placeholder: "Aye",
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
                                                        placeholder: "Nay",
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
                                                div {
                                                    style: "
                                                        display: flex;
                                                        gap: 4px;
                                                    ",
                                                    Input {
                                                        message: vote.aye / KUSAMA_PRECISION_DECIMALS,
                                                        size: ElementSize::Small,
                                                        placeholder: "Aye",
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
                                                        placeholder: "Nay",
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
                                                        placeholder: "Abstain",
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
