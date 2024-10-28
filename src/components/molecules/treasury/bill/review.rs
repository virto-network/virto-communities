use dioxus::prelude::*;

use crate::components::atoms::icon_button::Variant as IconButtonVariant;
use crate::components::atoms::{
    dropdown::ElementSize, key_value::Variant as KeyValueVariant, Button, ChevronLeft,
    ChevronRight, Icon, KeyValue,
};
use crate::components::atoms::{Close, FileLine, IconButton};

use crate::pages::bill::BillStep;

#[component]
pub fn ReviewForm() -> Element {
    let mut onboarding_step = use_context::<Signal<BillStep>>();
    rsx!(
            section { class: "bill__form",
                h3 {class: "send__form__title", "Review & Pay"}
                hr {class: "divider divider--item"}
                div { class: "send__form__transfer",
                    p { class: "send__form__details",
                        {format!("{} to {}", "Bank Transfer", "Debug LLC")}
                    }
                    p { class: "send__form__amount",
                        {format!("${} USDT", "220")}
                    }
                }
                div { class: "column",
                    KeyValue {
                        class: "key-value",
                        is_spaced: true,
                        text: "Recipient".to_string(),
                        size: ElementSize::Medium,
                        variant: KeyValueVariant::Secondary,
                        body: rsx!(
                            "David Barinas"
                        )
                    }
                    KeyValue {
                        class: "key-value",
                        is_spaced: true,
                        text: "Bank".to_string(),
                        size: ElementSize::Medium,
                        variant: KeyValueVariant::Secondary,
                        body: rsx!(
                            "Bancolombia"
                        )
                    }
                    KeyValue {
                        class: "key-value",
                        is_spaced: true,
                        text: "Account no.".to_string(),
                        size: ElementSize::Medium,
                        variant: KeyValueVariant::Secondary,
                        body: rsx!(
                            "123456789123456789"
                        )
                    }
                    KeyValue {
                        class: "key-value",
                        is_spaced: true,
                        text: "Account type".to_string(),
                        size: ElementSize::Medium,
                        variant: KeyValueVariant::Secondary,
                        body: rsx!(
                            "Savings"
                        )
                    }
                }
                hr {class: "divider divider--item"}
                div { class: "column",
                    KeyValue {
                        class: "key-value",
                        is_spaced: true,
                        text: "From".to_string(),
                        size: ElementSize::Medium,
                        variant: KeyValueVariant::Secondary,
                        body: rsx!(
                            "Tunja Community"
                        )
                    }
                    KeyValue {
                        class: "key-value",
                        is_spaced: true,
                        text: "Account".to_string(),
                        size: ElementSize::Medium,
                        variant: KeyValueVariant::Secondary,
                        body: rsx!(
                            "Checking"
                        )
                    }
                    KeyValue {
                        class: "key-value",
                        is_spaced: true,
                        text: "Initiated on".to_string(),
                        size: ElementSize::Medium,
                        variant: KeyValueVariant::Secondary,
                        body: rsx!(
                            "September 23, 2024"
                        )
                    }
                    KeyValue {
                        class: "key-value",
                        is_spaced: true,
                        text: "Send on".to_string(),
                        size: ElementSize::Medium,
                        variant: KeyValueVariant::Secondary,
                        body: rsx!(
                            "October 1, 2024"
                        )
                    }
                }
                hr {class: "divider divider--item"}
                div { class: "column",
                    KeyValue {
                        class: "key-value",
                        is_spaced: true,
                        text: "Memo for the recipient".to_string(),
                        size: ElementSize::Medium,
                        variant: KeyValueVariant::Secondary,
                        body: rsx!(
                            "From Virto test"
                        )
                    }
                }
                hr {class: "divider divider--item"}
                div {
                    span { class: "input__label",
                        "Email receipt to (optional)"
                    }
                    div {
                        class: "tag tag--bill",
                        button {
                            class: "tag__text",
                            onclick: move |_| {
        
                            },
                            "xeno@yandex.ru"
                        }
                        IconButton {
                            class: "button--drop bg--transparent",
                            body: rsx!(
                                Icon {
                                    icon: Close,
                                    height: 20,
                                    width: 20,
                                    fill: "var(--fill-400)"
                                }
                            ),
                            on_click: move |_| {
        
                            }
                        }
                    }
                }
                hr {class: "divider divider--item"}
                div { class: "column",
                    KeyValue {
                        class: "key-value",
                        is_spaced: true,
                        text: "Gl Code".to_string(),
                        size: ElementSize::Medium,
                        variant: KeyValueVariant::Secondary,
                        body: rsx!(
                            "120 - Accounts receivable"
                        )
                    }
                    KeyValue {
                        class: "key-value",
                        is_spaced: true,
                        text: "Notes".to_string(),
                        size: ElementSize::Medium,
                        variant: KeyValueVariant::Secondary,
                        body: rsx!(
                            "This is a note"
                        )
                    }
                    KeyValue {
                        class: "key-value",
                        is_spaced: true,
                        text: "Attachments".to_string(),
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
                        text: "Notes",
                        size: ElementSize::Small,
                        left_icon: rsx! {
                            Icon { icon: ChevronLeft, height: 20, width: 20, fill: "var(--base-fill-5)" }
                        },
                        status: None,
                        on_click: move |_| {
                            onboarding_step.set(BillStep::Info);
                        }
                    }
                    Button {
                        text: "Pay",
                        size: ElementSize::Small,
                        right_icon: rsx! {
                            Icon { icon: ChevronRight, height: 20, width: 20, fill: "var(--base-fill-5)" }
                        },
                        status: None,
                        on_click: move |_| {
                            onboarding_step.set(BillStep::Completed);
                        }
                    }
                }
            }
        )
}
