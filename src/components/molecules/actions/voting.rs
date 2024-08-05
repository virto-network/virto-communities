use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::atoms::{
        dropdown::{DropdownItem, ElementSize},
        icon_button::Variant,
        AddPlus, Dropdown, Icon, IconButton, Input, MinusCircle,
    },
    hooks::use_initiative::VotingOpenGovAction,
};

#[derive(PartialEq, Props, Clone)]
pub struct VotingProps {
    meta: VotingOpenGovAction,
}

pub fn Voting(props: VotingProps) -> Element {
    let i18 = use_i18();

    // let mut option_value = use_signal(|| props.value.option.clone());
    // let mut input_value = use_signal::<String>(|| props.value.input.clone());

    let mut items = vec![];
    let dropdown_options = vec![
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
            value: "Split Abstain".to_string(),
        },
    ];

    for account in dropdown_options.clone().into_iter() {
        items.push(rsx!(span {
            "{account.value}"
        }))
    }

    rsx!(
        ul { class: "form__inputs form__inputs--combo",
            {
                props.meta.proposals.iter().enumerate().map(|(index_meta, proposal)| {
                    rsx!(
                        li {
                            Dropdown {
                                class: "dropdown--left".to_string(),
                                value: None,
                                placeholder: translate!(i18, "header.cta.account"),
                                size: ElementSize::Small,
                                default: None,
                                on_change: move |event: usize| {

                                },
                                body: items.clone()
                            }
                            Input {
                                message: "",
                                size: ElementSize::Small,
                                placeholder: "Poll Index",
                                error: None,
                                on_input: move |event: Event<FormData>| {

                                },
                                on_keypress: move |_| {},
                                on_click: move |_| {},
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
                                    // if let ActionItem::KusamaTreasury(ref mut meta) = initiative.get_action(index) {
                                    //     meta.periods.remove(index_meta);
                                    //     initiative.update_action(index, ActionItem::KusamaTreasury(meta.clone()));
                                    // }
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
                    // if let ActionItem::KusamaTreasury(ref mut meta) = initiative.get_action(index) {
                    //     meta.add_period(KusamaTreasury::default());
                    //     initiative.update_action(index, ActionItem::KusamaTreasury(meta.clone()));
                    // }
                }
            }
        }
    )
}
