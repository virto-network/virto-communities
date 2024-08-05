use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::{
        atoms::{
            combo_input::{ComboInputOption, ComboInputValue},
            dropdown::{DropdownItem, ElementSize},
            icon_button::Variant,
            AddPlus, ComboInput, Dropdown, Icon, IconButton, Input, MinusCircle, SubstractLine,
        },
        molecules::Voting,
    },
    hooks::{
        use_initiative::{
            use_initiative, ActionItem, AddMembersAction, KusamaTreasury, MediumOptions, MemberItem,
        },
        use_notification::use_notification,
        use_spaces_client::use_spaces_client,
    },
};

const KUSAMA_PRECISION_DECIMALS: u64 = 1_000_000_000_000;

#[component]
pub fn InitiativeActions() -> Element {
    let i18 = use_i18();
    let mut initiative = use_initiative();
    let mut notification = use_notification();
    let spaces_client = use_spaces_client();

    let actions_lock = initiative.get_actions();

    let mut items = vec![];

    for option in initiative.get_actions_options().into_iter() {
        items.push(rsx!(span {
            "{option.value}"
        }))
    }

    rsx!(
        ul { class: "form__inputs form__inputs--initiative",
            {
                actions_lock.iter().enumerate().map(|(index, action)| {
                    rsx!(
                        li {
                            div { class: "form__input",
                                IconButton {
                                    variant: Variant::Round,
                                    size: ElementSize::Small,
                                    class: "button--action",
                                    body: rsx!(
                                        Icon {
                                            icon: SubstractLine,
                                            height: 24,
                                            width: 24,
                                            fill: "var(--fill-00)"
                                        }
                                    ),
                                    on_click: move |_| {
                                        initiative.remove_action(index);
                                    }
                                }
                                Dropdown {
                                    class: "action__option dropdown--left".to_string(),
                                    value: Some(action.option()),
                                    placeholder: translate!(i18, "header.cta.account"),
                                    size: ElementSize::Small,
                                    default: None,
                                    on_change: move |event: usize| {
                                        let options = initiative.get_actions_options();

                                        let to_assign = &options[event];

                                        initiative.update_action(index, initiative.to_action_option(to_assign.key.clone()));
                                    },
                                    body: items.clone()
                                }
                            }
                            match action {
                                ActionItem::AddMembers(meta) =>  {
                                    rsx!(
                                        ul { class: "form__inputs form__inputs--combo",
                                            {
                                                meta.members.iter().enumerate().map(|(index_meta, member)| {
                                                    let dropdown_item = DropdownItem { key: match member.medium {
                                                        MediumOptions::Wallet => translate!(i18, "onboard.invite.form.wallet.label"),
                                                    }, value: match member.medium.clone() {
                                                        MediumOptions::Wallet => translate!(i18, "onboard.invite.form.wallet.label"),
                                                    } };

                                                    rsx!(
                                                        li {
                                                            ComboInput {
                                                                size: ElementSize::Small,
                                                                value: ComboInputValue {
                                                                    option: ComboInputOption::Dropdown(dropdown_item),
                                                                    input: member.account.clone()
                                                                },
                                                                placeholder: match member.medium {
                                                                    MediumOptions::Wallet => translate!(i18, "onboard.invite.form.wallet.placeholder"),
                                                                },
                                                                on_change: move |event: ComboInputValue| {
                                                                    let medium = match event.option {
                                                                        ComboInputOption::Dropdown(value) => {
                                                                            match value.key.as_str() {
                                                                                "Wallet" => MediumOptions::Wallet,
                                                                                _ => todo!()
                                                                            }
                                                                        },
                                                                        _ => todo!()
                                                                    };
                                                                    if let ActionItem::AddMembers(ref mut meta) = initiative.get_action(index) {
                                                                        meta.members[index_meta] = MemberItem { medium, account: event.input };
                                                                        initiative.update_action(index, ActionItem::AddMembers(meta.clone()));
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
                                                                    if let ActionItem::AddMembers(ref mut meta) = initiative.get_action(index) {
                                                                        meta.members.remove(index_meta);
                                                                        initiative.update_action(index, ActionItem::AddMembers(meta.clone()));
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
                                                    if let ActionItem::AddMembers(ref mut meta) = initiative.get_action(index) {
                                                        meta.add_member(MemberItem::default());
                                                        initiative.update_action(index, ActionItem::AddMembers(meta.clone()));
                                                    }
                                                }
                                            }
                                        }
                                    )
                                },
                                ActionItem::KusamaTreasury(meta) =>  {
                                    rsx!(
                                        ul { class: "form__inputs form__inputs--combo",
                                            p { class: "form__inputs__disclaimer",
                                                {translate!(i18, "initiative.steps.actions.kusama_treasury.disclaimer.period_1")}
                                                if meta.periods.len() > 1 {
                                                    {translate!(i18, "initiative.steps.actions.kusama_treasury.disclaimer.period_n")}
                                                }
                                            }
                                            {
                                                meta.periods.iter().enumerate().map(|(index_meta, period)| {
                                                    let option = if index_meta == 0 {
                                                        ComboInputOption::None
                                                    } else {
                                                        ComboInputOption::Date(period.date.clone())
                                                    };

                                                    rsx!(
                                                        li {
                                                            ComboInput {
                                                                class: "combo-input--reverse",
                                                                size: ElementSize::Small,
                                                                value: ComboInputValue {
                                                                    option,
                                                                    input: (period.amount / KUSAMA_PRECISION_DECIMALS).to_string()
                                                                },
                                                                error: if period.amount <= 0 {
                                                                    Some(translate!(i18, "initiative.steps.actions.kusama_treasury.error"))
                                                                } else {
                                                                    None
                                                                },
                                                                placeholder: translate!(i18, "initiative.steps.actions.kusama_treasury.placeholder"),
                                                                right_text: {
                                                                    rsx!(
                                                                        span { class: "input--right__text",
                                                                            "KSM"
                                                                        }
                                                                    )
                                                                },
                                                                on_change: move |event: ComboInputValue| {
                                                                    let date = match event.option {
                                                                        ComboInputOption::Date(value) => {
                                                                          value
                                                                        },
                                                                        ComboInputOption::None => {
                                                                            "".into()
                                                                        }
                                                                        _ => todo!()
                                                                    };
                                                                    if let ActionItem::KusamaTreasury(ref mut meta) = initiative.get_action(index) {
                                                                        // Scale amount
                                                                        let amount = event.input.parse::<f64>().unwrap_or(0.0);
                                                                        let scaled_amount = amount * KUSAMA_PRECISION_DECIMALS as f64;
                                                                        meta.periods[index_meta] = KusamaTreasury { date, amount: scaled_amount as u64 };
                                                                        initiative.update_action(index, ActionItem::KusamaTreasury(meta.clone()));
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
                                                                    if let ActionItem::KusamaTreasury(ref mut meta) = initiative.get_action(index) {
                                                                        meta.periods.remove(index_meta);
                                                                        initiative.update_action(index, ActionItem::KusamaTreasury(meta.clone()));
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
                                                    if let ActionItem::KusamaTreasury(ref mut meta) = initiative.get_action(index) {
                                                        meta.add_period(KusamaTreasury::default());
                                                        initiative.update_action(index, ActionItem::KusamaTreasury(meta.clone()));
                                                    }
                                                }
                                            }
                                        }
                                    )
                                }
                                ActionItem::VotingOpenGov(meta) =>  {
                                    rsx!(
                                        Voting {
                                            meta: meta.clone()
                                        }
                                    )
                                }
                            }
                        }
                    )
                })
            }
            li {
                div { class: "form__input",
                    Dropdown {
                        class: "action__option dropdown--left".to_string(),
                        value: None,
                        placeholder: translate!(i18, "header.cta.account"),
                        size: ElementSize::Small,
                        default: None,
                        on_change: move |event: usize| {
                            // let to_assign = &dropdown_options[event];

                            // dropdown_value.set(Some(to_assign.clone()));
                        },
                        body: items
                    }
                    IconButton {
                        variant: Variant::Round,
                        size: ElementSize::Small,
                        class: "button--action",
                        body: rsx!(
                            Icon {
                                icon: AddPlus,
                                height: 24,
                                width: 24,
                                fill: "var(--fill-00)"
                            }
                        ),
                        on_click: move |_| {
                            initiative.push_action(ActionItem::AddMembers(AddMembersAction { members: vec![MemberItem { medium: MediumOptions::Wallet, account: String::from("") }] }));
                        }
                    }
                }
            }
        }
    )
}
