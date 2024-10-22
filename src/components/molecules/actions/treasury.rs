use crate::{
    components::atoms::{
        dropdown::{DropdownItem, ElementSize},
        icon_button::Variant,
        input::InputType,
        AddPlus, Icon, IconButton, Input, MinusCircle,
    },
    hooks::use_initiative::{use_initiative, ActionItem, KusamaTreasury, KusamaTreasuryAction},
};
use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
#[derive(PartialEq, Props, Clone)]
pub struct VotingProps {
    index: usize,
    meta: KusamaTreasuryAction,
}
const KUSAMA_PRECISION_DECIMALS: u64 = 1_000_000_000_000;
pub fn TreasuryAction(props: VotingProps) -> Element {
    let i18 = use_i18();
    let mut initiative = use_initiative();
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
        items.push(rsx!(
            span { "{account.value}" }
        ))
    }
    rsx!(
        ul { class: "form__inputs form__inputs--combo",
            p { class: "form__inputs__disclaimer",
                {translate!(i18, "initiative.steps.actions.kusama_treasury.disclaimer.period_1")},
                if props.meta.periods.len() > 1 {
                    { translate!(i18,
                    "initiative.steps.actions.kusama_treasury.disclaimer.period_n") }
                }
            }
            {
                props.meta.periods.iter().enumerate().map(|(index_meta, period)| {
                    rsx!(
                        li {
                            div {
                                style: "
                                    width: 100%;
                                    display: flex;
                                    gap: 4px;
                                ",
                                Input {
                                    message: (period.amount / KUSAMA_PRECISION_DECIMALS).to_string(),
                                    size: ElementSize::Small,
                                    placeholder: translate!(i18, "initiative.steps.actions.kusama_treasury.placeholder"),
                                    error: {
                                        if period.amount > 0 {
                                            None
                                        } else {
                                            Some("Amount should be greater than 0".to_string())
                                        }
                                    },
                                    right_text: {
                                        rsx!(
                                            span { class: "input--right__text",
                                                "KSM"
                                            }
                                        )
                                    },
                                    on_input: move |event: Event<FormData>| {
                                        if let ActionItem::KusamaTreasury(ref mut meta) = initiative.get_action(props.index) {
                                            // Scale amount
                                            let amount = event.value().parse::<f64>().unwrap_or(0.0);
                                            let scaled_amount = amount * KUSAMA_PRECISION_DECIMALS as f64;
                                            meta.periods[index_meta].amount = scaled_amount as u64 ;
                                            initiative.update_action(props.index, ActionItem::KusamaTreasury(meta.clone()));
                                        }
                                    },
                                    on_keypress: move |_| {},
                                    on_click: move |_| {},
                                }
                                if index_meta > 0 {
                                    Input {
                                        message: period.date.clone(),
                                        size: ElementSize::Small,
                                        itype: InputType::Date,
                                        placeholder: translate!(i18, "initiative.steps.actions.kusama_treasury.placeholder"),
                                        error: None,
                                        on_input: move |event: Event<FormData>| {
                                            if let ActionItem::KusamaTreasury(ref mut meta) = initiative.get_action(props.index) {
                                                meta.periods[index_meta].date = event.value() ;
                                                initiative.update_action(props.index, ActionItem::KusamaTreasury(meta.clone()));
                                            }
                                        },
                                        on_keypress: move |_| {},
                                        on_click: move |_| {},
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
                                    if let ActionItem::KusamaTreasury(ref mut meta) = initiative.get_action(props.index) {
                                        meta.periods.remove(index_meta);
                                        initiative.update_action(props.index, ActionItem::KusamaTreasury(meta.clone()));
                                    }
                                }
                            }
                        }
                    )
                })
            },
            IconButton {
                variant: Variant::Round,
                size: ElementSize::Small,
                class: "button--avatar",
                body: rsx!(
                    Icon { icon : AddPlus, height : 24, width : 24, fill :
                    "var(--state-primary-active)" }
                ),
                on_click: move |_| {
                    if let ActionItem::KusamaTreasury(ref mut meta) = initiative
                        .get_action(props.index)
                    {
                        meta.add_period(KusamaTreasury::default());
                        initiative
                            .update_action(props.index, ActionItem::KusamaTreasury(meta.clone()));
                    }
                }
            }
        }
    )
}
