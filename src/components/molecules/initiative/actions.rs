use crate::{
    components::{
        atoms::{
            dropdown::ElementSize, icon_button::Variant, AddPlus, Dropdown, Icon, IconButton,
            SubstractLine,
        },
        molecules::{MembersAction, TransferAction, TreasuryAction, VotingAction},
    },
    hooks::use_initiative::{
        use_initiative, ActionItem, AddMembersAction, MediumOptions, MemberItem,
    },
};
use dioxus::prelude::*;
use dioxus_i18n::t;
#[component]
pub fn InitiativeActions() -> Element {
    
    let mut initiative = use_initiative();
    let actions_lock = initiative.get_actions();
    let mut items = vec![];
    for option in initiative.get_actions_options().into_iter() {
        items.push(rsx!(
            span { "{option.value}" }
        ))
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
                                    placeholder: t!("header-cta-account"),
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
                                        MembersAction {
                                            index: index,
                                            meta: meta.clone()
                                        }
                                    )
                                },
                                ActionItem::KusamaTreasury(meta) =>  {
                                    rsx!(
                                        TreasuryAction {
                                            index: index,
                                            meta: meta.clone()
                                        }
                                    )
                                }
                                ActionItem::VotingOpenGov(meta) =>  {
                                    rsx!(
                                        VotingAction {
                                            index: index,
                                            meta: meta.clone()
                                        }
                                    )
                                }
                                ActionItem::CommunityTransfer(meta) =>  {
                                    rsx!(
                                        TransferAction {
                                            index: index,
                                            meta: meta.clone()
                                        }
                                    )
                                }
                            }
                        }
                    )
                })
            },
            li {
                div { class: "form__input",
                    Dropdown {
                        class: "action__option dropdown--left".to_string(),
                        value: None,
                        placeholder: t!("header-cta-account"),
                        size: ElementSize::Small,
                        default: None,
                        on_change: move |event: usize| {
                            let options = initiative.get_actions_options();
                            let to_assign = &options[event];
                            let action = initiative.to_action_option(to_assign.key.clone());
                            initiative.push_action(action);
                        },
                        body: items
                    }
                    IconButton {
                        variant: Variant::Round,
                        size: ElementSize::Small,
                        class: "button--action",
                        disabled: actions_lock.is_empty(),
                        body: rsx! {
                            Icon { icon: AddPlus, height: 24, width: 24, fill: "var(--fill-00)" }
                        },
                        on_click: move |_| {
                            initiative
                                .push_action(
                                    ActionItem::AddMembers(AddMembersAction {
                                        members: vec![
                                            MemberItem {
                                                medium: MediumOptions::Wallet,
                                                account: String::from(""),
                                            },
                                        ],
                                    }),
                                );
                        }
                    }
                }
            }
        }
    )
}
