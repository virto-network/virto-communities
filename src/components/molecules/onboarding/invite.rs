use dioxus::prelude::*;
use dioxus_i18n::t;
use crate::{
    components::atoms::{
        combo_input::{ComboInputOption, ComboInputValue},
        dropdown::{DropdownItem, ElementSize},
        icon_button::Variant, AddPlus, ComboInput, Icon, IconButton, MinusCircle, Title,
    },
    hooks::use_onboard::{use_onboard, InvitationItem, MediumOptions},
};
#[component]
pub fn OnboardingInvite() -> Element {
    
    let mut onboard = use_onboard();
    let members_lock = onboard.get_invitations();
    let mut to_pay = consume_context::<Signal<f64>>();
    use_effect(
        use_reactive(
            &onboard.get_invitations().len(),
            move |_| {
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
            },
        ),
    );
    rsx!(
        div { class: "form__title",
            span { class: "label", {t!("onboard-invite-label")} }
            Title { text: t!("onboard-invite-title") }
        }
        ul { class: "form__inputs",
            {
                members_lock.iter().enumerate().map(|(index, member)| {
                    let x  = DropdownItem { key: match member.medium {
                        MediumOptions::Wallet => t!("onboard-invite-form-wallet-label"),
                    }, value: match member.medium.clone() {
                        MediumOptions::Wallet => t!("onboard-invite-form-wallet-label"),
                    } };
            
                    rsx!(
                        li {
                            ComboInput {
                                size: ElementSize::Big,
                                value: ComboInputValue {
                                    option: ComboInputOption::Dropdown(x),
                                    input: member.account.clone()
                                },
                                placeholder: match member.medium {
                                    MediumOptions::Wallet => t!("onboard-invite-form-wallet-placeholder"),
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
            },
            IconButton {
                variant: Variant::Round,
                size: ElementSize::Medium,
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
                    onboard
                        .push_invitation(InvitationItem {
                            medium: MediumOptions::Wallet,
                            account: String::from(""),
                        });
                }
            }
        }
    )
}
