use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::atoms::{
        combo_input::ComboInputValue,
        dropdown::{DropdownItem, ElementSize},
        icon_button::Variant,
        AddPlus, Attach, ComboInput, Icon, IconButton, Input, MinusCircle, TextareaInput, Title,
    },
    hooks::use_onboard::{use_onboard, InvitationItem, Invitations, MediumOptions},
};

#[component]
pub fn OnboardingInvite() -> Element {
    let i18 = use_i18();
    let mut onboard = use_onboard();

    let members_lock = onboard.get_invitations();
    let mut to_pay = consume_context::<Signal<f64>>();

    use_effect(use_reactive(&onboard.get_invitations().len(), move |_| {
        let members = onboard
            .get_invitations()
            .into_iter()
            .filter_map(|invitation| {
                if !invitation.account.is_empty() {
                    Some(invitation.account)
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();

        to_pay.set(0.51 + 0.11 + members.len() as f64 * 0.3)
    }));

    rsx!(
        div { class: "form__title",
            span { class: "label",
                {translate!(i18, "onboard.invite.label")}
            }
            Title {
                text: translate!(i18, "onboard.invite.title")
            }
        }
        ul { class: "form__inputs",
            {
                members_lock.iter().enumerate().map(|(index, member)| {
                    rsx!(
                        li {
                            ComboInput {
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

                                    onboard.update_invitation(index, InvitationItem { medium, account: event.input });
                                }
                            }
                            IconButton {
                                variant: Variant::Round,
                                size: ElementSize::Medium,
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
                                    onboard.remove_invitation(index);
                                }
                            }
                        }
                    )
                })
            }
            IconButton {
                variant: Variant::Round,
                size: ElementSize::Medium,
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
                    onboard.push_invitation(InvitationItem {
                        medium: MediumOptions::Wallet,
                        account: String::from(""),
                    });
                 }
            }
        }
    )
}
