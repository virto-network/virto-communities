use dioxus::prelude::*;

use crate::{
    components::atoms::{
        dropdown::ElementSize, icon_button::Variant as IconButtonVariant, Badge, Button,
        CheckboxCircleFill, Close, Download, Icon, IconButton,
    },
    pages::invoice::InvoiceStep,
};

#[component]
pub fn Completed() -> Element {
    let mut onboarding_step = use_context::<Signal<InvoiceStep>>();
    rsx!(
        section { class: "send__form",
            Icon { icon: CheckboxCircleFill, height: 32, width: 32, fill: "var(--base-fill-5)" }
            h3 {class: "send__form__confirm", "Your Invoice has been created and sent!"}
            section { class: "send__form__transfer__wrapper",
                div { class: "send__form__transfer__details",
                    div { class: "send__form__transfer",
                        p { class: "send__form__details",
                            {format!("{} to {}", "Bank Transfer", "Debug LLC")}
                        }
                        p { class: "send__form__amount",
                            {format!("${} USDT", "220")}
                        }
                    }
                    Badge { class : "badge--green-light", text : "Due on Nov 9, 2024" }
                }
                hr {class:"divider divider--table"}
                div { class: "send__form__transfer__dispatch",
                    div {
                        span { class: "input__label",
                            "Invoice sent to"
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
                    div {
                        span { class: "input__label",
                            "Invoice payment link"
                        }
                        div { class: "bill__completed__link",
                            div {
                                class: "tag tag--bill",
                                button {
                                    class: "tag__text",
                                    onclick: move |_| {

                                    },
                                    "https://virto.network/invoice/go/lkewiow91TIB4235f23fd"
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
                            IconButton {
                                variant: IconButtonVariant::Ghost,
                                size: ElementSize::Small,
                                class: "button--avatar",
                                body: rsx!(Icon { icon : Download, height : 20, width : 20, fill : "var(--state-primary-active)" }),
                                on_click: move |_| {
                                }
                            }
                        }
                    }
                }
            }
            div { class: "row",
                Button {
                    text: "Send Another Invoice",
                    size: ElementSize::Small,
                    status: None,
                    on_click: move |_| {
                        onboarding_step.set(InvoiceStep::Recipient);
                    }
                }
                Button {
                    text: "Return to virto home",
                    size: ElementSize::Small,
                    status: None,
                    on_click: move |_| {
                        onboarding_step.set(InvoiceStep::Completed);
                    }
                }
            }
        }
    )
}
