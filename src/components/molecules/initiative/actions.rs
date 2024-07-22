use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::atoms::{
        combo_input::ComboInputValue,
        dropdown::{DropdownItem, ElementSize},
        icon_button::Variant,
        AddPlus, ComboInput, Dropdown, Icon, IconButton, MinusCircle, SubstractLine,
    },
    hooks::{
        use_initiative::{use_initiative, ActionItem, AddMembersAction, MediumOptions, MemberItem},
        use_notification::use_notification,
        use_spaces_client::use_spaces_client,
    },
};

#[component]
pub fn InitiativeActions() -> Element {
    let i18 = use_i18();
    let mut initiative = use_initiative();
    let mut notification = use_notification();
    let spaces_client = use_spaces_client();

    let mut name_maxlength = use_signal(|| 24);

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
                                                    rsx!(
                                                        li {
                                                            ComboInput {
                                                                size: ElementSize::Small,
                                                                value: ComboInputValue { dropdown: DropdownItem { key: match member.medium {
                                                                    MediumOptions::Wallet => translate!(i18, "onboard.invite.form.wallet.label"),
                                                                    MediumOptions::Email => translate!(i18, "onboard.invite.form.email.label"),
                                                                    MediumOptions::Telegram => translate!(i18, "onboard.invite.form.phone.label"),
                                                                }, value: match member.medium.clone() {
                                                                    MediumOptions::Wallet => translate!(i18, "onboard.invite.form.wallet.label"),
                                                                    MediumOptions::Email => translate!(i18, "onboard.invite.form.email.label"),
                                                                    MediumOptions::Telegram => translate!(i18, "onboard.invite.form.phone.label"),
                                                                } }, input: member.account.clone() },
                                                                placeholder: match member.medium {
                                                                    MediumOptions::Wallet => translate!(i18, "onboard.invite.form.wallet.placeholder"),
                                                                    MediumOptions::Email => translate!(i18, "onboard.invite.form.email.placeholder"),
                                                                    MediumOptions::Telegram => translate!(i18, "onboard.invite.form.phone.placeholder"),
                                                                },
                                                                on_change: move |event: ComboInputValue| {
                                                                    log::info!("{:?}", event.dropdown.key);
                                                                    let medium = match event.dropdown.key.as_str() {
                                                                        "Wallet" => MediumOptions::Wallet,
                                                                        "Email" => MediumOptions::Email,
                                                                        "Telegram" => MediumOptions::Telegram,
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
                                _ =>  {
                                    rsx!(
                                        span {""}
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
