use dioxus::prelude::*;

use crate::{
    components::atoms::{
        button::Variant, dropdown::ElementSize, Badge, Button, ChevronLeft, ChevronRight, Close,
        Icon, IconButton,
    },
    pages::invoice::InvoiceStep,
};

#[component]
pub fn ReviewForm() -> Element {
    let mut onboarding_step = use_context::<Signal<InvoiceStep>>();
    rsx!(
        section { class: "bill__form",
            h3 {class: "send__form__title", "Review and send"}
            div { class: "bill__form__review__section",
                "INVOICE DETAILS"
                hr {class: "divider divider--title"}
            }
            div { class: "bill__form__transfer__details",
                div { class: "send__form__transfer",
                    p { class: "send__form__details",
                        {format!("{} to {}", "Invoice", "David Barinas")}
                    }
                    p { class: "send__form__amount",
                        {format!("${} USDT", "220")}
                    }
                }
                Badge { class : "badge--green-light", text : "Recurring" }
            }
            div { class: "information",
                p { class: "information__title",
                    "Send weekly on Thursday until 12 invoices are sent."
                }
            }
            div { class: "bill__form__review__section",
                "SEND INVOICE TO"
                hr {class: "divider divider--title"}
            }
            div {
                span { class: "input__label",
                    "Email to"
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
            div { class: "bill__form__review__cta",
                Button {
                    text: "Back",
                    size: ElementSize::Small,
                    variant: Variant::Tertiary,
                    left_icon: rsx! {
                        Icon { icon: ChevronLeft, height: 20, width: 20, fill: "var(--base-fill-5)" }
                    },
                    status: None,
                    on_click: move |_| {
                        onboarding_step.set(InvoiceStep::PaymentMethod);
                    }
                }
                div { class: "row xl-3",
                    Button {
                        text: "Create only",
                        size: ElementSize::Small,
                        status: None,
                        on_click: move |_| {
                            onboarding_step.set(InvoiceStep::Completed);
                        }
                    }
                    Button {
                        text: "Create & Send Email",
                        size: ElementSize::Small,
                        right_icon: rsx! {
                            Icon { icon: ChevronRight, height: 20, width: 20, fill: "var(--base-fill-5)" }
                        },
                        status: None,
                        on_click: move |_| {
                            onboarding_step.set(InvoiceStep::Completed);
                        }
                    }
                }
            }
        }
    )
}
