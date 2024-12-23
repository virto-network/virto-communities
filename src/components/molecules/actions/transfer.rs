use sp_core::crypto::Ss58Codec;

use crate::{
    components::atoms::{
        dropdown::ElementSize, icon_button::Variant, AddPlus, Icon, IconButton,
        Input, MinusCircle,
    },
    hooks::use_initiative::{use_initiative, ActionItem, CommunityTransferAction, TransferItem},
};
use dioxus::prelude::*;
use dioxus_i18n::t;
#[derive(PartialEq, Props, Clone)]
pub struct VotingProps {
    index: usize,
    meta: CommunityTransferAction,
}
const KUSAMA_PRECISION_DECIMALS: u64 = 1_000_000_000_000;
pub fn TransferAction(props: VotingProps) -> Element {
    
    let mut initiative = use_initiative();
    rsx!(
        ul { class: "form__inputs form__inputs--combo",
            {
                props.meta.transfers.iter().enumerate().map(|(index_meta, transfer)| {
                    rsx!(
                        li {
                            div {
                                style: "
                                    width: 100%;
                                    display: flex;
                                    gap: 4px;
                                ",
                                Input {
                                    message: transfer.account.clone(),
                                    size: ElementSize::Small,
                                    placeholder: t!("initiative-steps-actions-community_transfer-dest-placeholder"),
                                    error: {
                                        match sp_core::sr25519::Public::from_ss58check(&transfer.account) {
                                            Ok(_) => None,
                                            Err(_) => Some(t!("initiative-steps-actions-error-invalid_address")),
                                        }
                                    },
                                    on_input: move |event: Event<FormData>| {
                                        if let ActionItem::CommunityTransfer(ref mut meta) = initiative.get_action(props.index) {
                                            meta.transfers[index_meta].account = event.value() ;
                                            initiative.update_action(props.index, ActionItem::CommunityTransfer(meta.clone()));
                                        }
                                    },
                                    on_keypress: move |_| {},
                                    on_click: move |_| {},
                                }
                                Input {
                                    message: (transfer.value / KUSAMA_PRECISION_DECIMALS).to_string(),
                                    size: ElementSize::Small,
                                    placeholder: t!("initiative-steps-actions-community_transfer-amount-placeholder"),
                                    error: {
                                        if transfer.value > 0 {
                                            None
                                        } else {
                                            Some(t!("initiative-steps-actions-error-amount"))
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
                                        if let ActionItem::CommunityTransfer(ref mut meta) = initiative.get_action(props.index) {
                                            // Scale amount
                                            let amount = event.value().parse::<f64>().unwrap_or(0.0);
                                            let scaled_amount = amount * KUSAMA_PRECISION_DECIMALS as f64;
                                            meta.transfers[index_meta].value = scaled_amount as u64 ;
                                            initiative.update_action(props.index, ActionItem::CommunityTransfer(meta.clone()));
                                        }
                                    },
                                    on_keypress: move |_| {},
                                    on_click: move |_| {},
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
                                    if let ActionItem::CommunityTransfer(ref mut meta) = initiative.get_action(props.index) {
                                        meta.transfers.remove(index_meta);
                                        initiative.update_action(props.index, ActionItem::CommunityTransfer(meta.clone()));
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
                    if let ActionItem::CommunityTransfer(ref mut meta) = initiative
                        .get_action(props.index)
                    {
                        meta.add_transfer(TransferItem::default());
                        initiative
                            .update_action(props.index, ActionItem::CommunityTransfer(meta.clone()));
                    }
                }
            }
        }
    )
}
