use dioxus::prelude::*;

use crate::{
    components::atoms::{
        autocomplete::AutocompleteItem, avatar::Variant, dropdown::ElementSize,
        icon_button::Variant as IconButtonVariant, Autocomplete, AutocompleteItemButton, Avatar,
        Button, Icon, IconButton, UploadExport,
    },
    hooks::use_recipients::use_recipients,
    pages::bill::BillStep,
};

#[component]
pub fn RecipientForm() -> Element {
    let mut onboarding_step = use_context::<Signal<BillStep>>();
    let mut dropdown_recipient_value = use_signal::<Option<AutocompleteItem>>(|| None);
    let mut filter_name = use_signal(|| String::new());
    let mut recipients = use_recipients();
    rsx!(
        section { class: "bill__form",
            h3 {class: "send__form__title", "Recipient match"}
            p { class: "send__form__description",
                "We were  able to find a match to your invoice based on bank details, name, email, and/or recipient address."
            }
            div { class: "recipient__info__wrapper",
                div { class: "recipient__info",
                    div { class: "recipient__card",
                        Avatar {
                            name: "DB",
                            size: 44,
                            uri: None,
                            variant: Variant::Round
                        }
                        div { class: "card-send2__info",
                            h5 { class: "card-send2__info__title",
                                "Alias"
                            }
                            p { class: "card-send2__info__description",
                                "Name"
                            }
                        }
                    }
                    IconButton {
                        variant: IconButtonVariant::Round,
                        size: ElementSize::Big,
                        class: "button--avatar bg--transparent",
                        body: rsx!(
                            Icon { icon : UploadExport, height : 20, width : 20, fill :
                            "var(--state-primary-active)" }
                        ),
                        on_click: move |_| {}
                    }
                }
                div { class: "recipient__address",
                    div { class: "recipient__card",
                        Icon { icon : UploadExport, height : 16, width : 16, fill :
                        "var(--state-primary-active)" }
                        div { class: "recipient__card__info",
                            h5 { class: "recipient__card__info__title",
                                "Recipient address"
                            }
                            p { class: "recipient__card__info__description",
                                "660 Mission St"
                            }
                            p { class: "recipient__card__info__description",
                                "San Francisco, CA"
                            }
                        }
                    }
                }
                hr {class: "divider"}
                div { class: "recipient__account",
                    div { class: "recipient__card",
                        Icon { icon : UploadExport, height : 16, width : 16, fill :
                        "var(--state-primary-active)" }
                        div { class: "recipient__card__info",
                            h5 { class: "recipient__card__info__title",
                                "Recipient account"
                            }
                            p { class: "recipient__card__info__description",
                                "ACH"
                            }
                            p { class: "recipient__card__info__description",
                                "Account no. *1234"
                            }
                        }
                    }
                }
            }
            div { class: "row",
                Button {
                    text: "Continue with Matched Recipient",
                    size: ElementSize::Small,
                    status: None,
                    on_click: move |_| {
                        onboarding_step.set(BillStep::PaymentMethod);
                    }
                }
            }
            div { class: "divider__text",
                hr {class: "divider"}
                span {"or"}
                hr {class: "divider"}
            }
            Autocomplete {
                value: dropdown_recipient_value(),
                placeholder: "",
                label: "Name",
                reverse: false,
                size: ElementSize::Small,
                default: None,
                on_change: move |event: usize| {
                    let binding = recipients.get_recipients_by_filters(Some(&filter_name()), None);
                    let recipient = binding.get(event);

                    if let Some(recipient) = recipient {
                        dropdown_recipient_value.set(Some(
                            AutocompleteItem { key: recipient.id.to_string(), value: recipient.alias.clone() }
                        ));
                        if let Ok(()) = recipients.set_recipient(recipient.id) {
                            onboarding_step.set(BillStep::PaymentMethod);
                        };
                    }
                },
                on_input: move |event: FormEvent| {
                    filter_name.set(event.value())
                },
                body: {
                    let filtered_recipients = recipients.get_recipients_by_filters(Some(&filter_name()), None).iter().map(|recipient| {
                        rsx!(AutocompleteItemButton {
                            title: recipient.alias.clone(),
                        })
                    }).collect();

                    filtered_recipients
                },
            }
        }
    )
}
