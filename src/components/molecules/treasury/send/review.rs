use dioxus::prelude::*;

use crate::{
    components::atoms::{
        dropdown::ElementSize, icon_button::Variant as IconButtonVariant,
        key_value::Variant as KeyValueVariant, Button, Close, FileLine, Icon, IconButton, KeyValue,
        TelegramLogo,
    },
    hooks::{use_recipients::use_recipients, use_send::use_send},
};

use super::recipient::ControlProps;

#[component]
pub fn ReviewForm(props: ControlProps) -> Element {
    let recipients = use_recipients();
    let send_transaction = use_send();
    rsx!(
        section { class: "send__form",
            h3 {class: "send__form__title", "Recipient Payment Details"}
            div { class: "send__form__transfer",
                p { class: "send__form__details",
                    {format!("{} to {}", send_transaction.get_data().method, recipients.get_recipient().name)}
                }
                p { class: "send__form__amount",
                    {format!("${} {}", send_transaction.get_data().amount, send_transaction.get_data().currency)}
                }
            }
            hr {class: "divider divider--item"}
            div { class: "column",
                KeyValue {
                    class: "key-value",
                    text: "Recipient".to_string(),
                    is_spaced: true,
                    size: ElementSize::Medium,
                    variant: KeyValueVariant::Secondary,
                    body: rsx!(
                        {recipients.get_recipient().name}
                    )
                }
                KeyValue {
                    class: "key-value",
                    text: "Bank".to_string(),
                    is_spaced: true,
                    size: ElementSize::Medium,
                    variant: KeyValueVariant::Secondary,
                    body: rsx!(
                        {send_transaction.get_data().bank}
                    )
                }
                KeyValue {
                    class: "key-value",
                    text: "Account no.".to_string(),
                    is_spaced: true,
                    size: ElementSize::Medium,
                    variant: KeyValueVariant::Secondary,
                    body: rsx!(
                        {send_transaction.get_data().account_number}
                    )
                }
                KeyValue {
                    class: "key-value",
                    text: "Account type".to_string(),
                    is_spaced: true,
                    size: ElementSize::Medium,
                    variant: KeyValueVariant::Secondary,
                    body: rsx!(
                        {send_transaction.get_data().account_type}
                    )
                }
            }
            hr {class: "divider divider--item"}
            div { class: "column",
                KeyValue {
                    class: "key-value",
                    text: "From".to_string(),
                    is_spaced: true,
                    size: ElementSize::Medium,
                    variant: KeyValueVariant::Secondary,
                    body: rsx!(
                        "Tunja Community"
                    )
                }
                KeyValue {
                    class: "key-value",
                    text: "Account".to_string(),
                    is_spaced: true,
                    size: ElementSize::Medium,
                    variant: KeyValueVariant::Secondary,
                    body: rsx!(
                        "Checking"
                    )
                }
                KeyValue {
                    class: "key-value",
                    text: "Initiated on".to_string(),
                    is_spaced: true,
                    size: ElementSize::Medium,
                    variant: KeyValueVariant::Secondary,
                    body: rsx!(
                        {send_transaction.get_data().payment_at}
                    )
                }
                KeyValue {
                    class: "key-value",
                    text: "Send on".to_string(),
                    is_spaced: true,
                    size: ElementSize::Medium,
                    variant: KeyValueVariant::Secondary,
                    body: rsx!(
                        {send_transaction.get_data().payment_at}
                    )
                }
            }
            hr {class: "divider divider--item"}
            div { class: "column",
                KeyValue {
                    class: "key-value",
                    text: "Memo for the recipient".to_string(),
                    is_spaced: true,
                    size: ElementSize::Medium,
                    variant: KeyValueVariant::Secondary,
                    body: rsx!(
                        {send_transaction.get_data().memo}
                    )
                }
            }
            hr {class: "divider divider--item"}
            div { class: "column",
                KeyValue {
                    class: "key-value",
                    text: "Gl Code".to_string(),
                    is_spaced: true,
                    size: ElementSize::Medium,
                    variant: KeyValueVariant::Secondary,
                    body: rsx!(
                        {send_transaction.get_data().payment_via}
                    )
                }
                KeyValue {
                    class: "key-value",
                    text: "Notes".to_string(),
                    is_spaced: true,
                    size: ElementSize::Medium,
                    variant: KeyValueVariant::Secondary,
                    body: rsx!(
                        {send_transaction.get_data().notes}
                    )
                }
                KeyValue {
                    class: "key-value",
                    text: "Attachments".to_string(),
                    is_spaced: true,
                    size: ElementSize::Medium,
                    variant: KeyValueVariant::Secondary,
                    body: rsx!(
                        section { class: "files-list",
                            div { class: "file-list__item",
                                Icon { icon: FileLine, height: 20, width: 20, fill: "var(--fill-400)" }
                                div { class: "file-list__wrapper",
                                    p { class: "file-list__item__name",
                                        "Filename.pdf"
                                    }
                                    IconButton {
                                        variant: IconButtonVariant::Round,
                                        size: ElementSize::Big,
                                        class: "button--avatar bg--transparent button--drop",
                                        body: rsx!(
                                            Icon { icon : Close, height : 20, width : 20, fill :
                                            "var(--fill-400)" }
                                        ),
                                        on_click: move |_| {  }
                                    }
                                }
                            }
                            div { class: "file-list__item",
                                Icon { icon: FileLine, height: 20, width: 20, fill: "var(--fill-400)" }
                                div { class: "file-list__wrapper",
                                    p { class: "file-list__item__name",
                                        "Filename.pdf"
                                    }
                                    IconButton {
                                        variant: IconButtonVariant::Round,
                                        size: ElementSize::Big,
                                        class: "button--avatar bg--transparent button--drop",
                                        body: rsx!(
                                            Icon { icon : Close, height : 20, width : 20, fill :
                                            "var(--fill-400)" }
                                        ),
                                        on_click: move |_| {  }
                                    }
                                }
                            }
                        }
                    )
                }
            }
            hr {class: "divider divider--item"}
            div { class: "row",
                Button {
                    text: "Cancel Transaction",
                    size: ElementSize::Small,
                    status: None,
                    on_click: move |_| {
                        props.on_back.call(())
                    }
                }
                Button {
                    text: "Send",
                    size: ElementSize::Small,
                    right_icon: rsx! {
                        Icon { icon: TelegramLogo, height: 20, width: 20, fill: "var(--base-surface-1)" }
                    },
                    status: None,
                    on_click: move |_| {
                        props.on_next.call(())
                    }
                }
            }
        }
    )
}
