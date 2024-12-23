use sp_core::crypto::Ss58Codec;

use crate::{
    components::atoms::{
        combo_input::{ComboInputOption, ComboInputValue},
        dropdown::{DropdownItem, ElementSize},
        icon_button::Variant,
        AddPlus, ComboInput, Icon, IconButton, MinusCircle,
    },
    hooks::use_initiative::{
        use_initiative, ActionItem, AddMembersAction, MediumOptions, MemberItem,
    },
};
use dioxus::prelude::*;
use dioxus_i18n::t;
#[derive(PartialEq, Props, Clone)]
pub struct VotingProps {
    index: usize,
    meta: AddMembersAction,
}
pub fn MembersAction(props: VotingProps) -> Element {
    
    let mut initiative = use_initiative();
    rsx!(
        ul { class: "form__inputs form__inputs--combo",
            {
                props.meta.members.iter().enumerate().map(|(index_meta, member)| {
                    let dropdown_item = DropdownItem { key: match member.medium {
                        MediumOptions::Wallet => t!("onboard-invite-form-wallet-label"),
                    }, value: match member.medium.clone() {
                        MediumOptions::Wallet => t!("onboard-invite-form-wallet-label"),
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
                                    MediumOptions::Wallet => t!("onboard-invite-form-wallet-placeholder"),
                                },
                                error: {
                                    match sp_core::sr25519::Public::from_ss58check(&member.account) {
                                        Ok(_) => None,
                                        Err(_) => Some(t!("onboard-invite-form-error-invalid_address")),
                                    }
                                },
                                on_change: move |event: ComboInputValue| {
                                    let ComboInputOption::Dropdown(value) = event.option else {
                                        return;
                                    };
                                
                                    let invite_wallet = t!("onboard-invite-form-wallet-label");
                                    let medium = if value.key == invite_wallet {
                                        MediumOptions::Wallet
                                    } else {
                                        return;
                                    };
                                    if let ActionItem::AddMembers(ref mut meta) = initiative.get_action(props.index) {
                                        meta.members[index_meta] = MemberItem { medium, account: event.input };
                                        initiative.update_action(props.index, ActionItem::AddMembers(meta.clone()));
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
                                    if let ActionItem::AddMembers(ref mut meta) = initiative.get_action(props.index) {
                                        meta.members.remove(index_meta);
                                        initiative.update_action(props.index, ActionItem::AddMembers(meta.clone()));
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
                body: rsx! {
                    Icon {
                        icon: AddPlus,
                        height: 24,
                        width: 24,
                        fill: "var(--state-primary-active)"
                    }
                },
                on_click: move |_| {
                    if let ActionItem::AddMembers(ref mut meta) = initiative.get_action(props.index)
                    {
                        meta.add_member(MemberItem::default());
                        initiative.update_action(props.index, ActionItem::AddMembers(meta.clone()));
                    }
                }
            }
        }
    )
}
