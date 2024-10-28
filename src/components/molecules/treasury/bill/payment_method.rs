use dioxus::prelude::*;

use crate::{
    components::atoms::{
        avatar::Variant,
        button::Variant as ButtonVariant,
        dropdown::{DropdownItem, ElementSize},
        input::InputType,
        AutocompleteItemButton, Avatar, Button, ChevronLeft, ChevronRight, Dropdown, Icon, Input,
        UploadExport,
    },
    hooks::use_bill::use_bill,
    pages::bill::BillStep,
};

#[component]
pub fn PaymentMethodForm() -> Element {
    let mut dropdown_payment_value = use_signal::<Option<DropdownItem>>(|| None);
    let mut payment_items = vec![];
    let mut onboarding_step = use_context::<Signal<BillStep>>();
    let mut bill_transaction = use_bill();

    let list_payment_items_mock = vec![DropdownItem {
        key: "bank".to_string(),
        value: "Bank Transfer".to_string(),
    }];
    for account in list_payment_items_mock.iter() {
        payment_items.push(rsx!(AutocompleteItemButton {
            title: account.value.clone(),
        }))
    }

    rsx!(
        section { class: "bill__form",
            h3 {class: "send__form__title", "Method and bill details"}
            Dropdown {
                class: "header__wallet".to_string(),
                value: dropdown_payment_value(),
                placeholder: "Select",
                label: "Payment Method",
                help: "1 business day • 5% fee",
                size: ElementSize::Small,
                default: None,
                on_change: move |event: usize| {
                    let option = &list_payment_items_mock[event];
                    dropdown_payment_value.set(Some(option.clone()));
                    bill_transaction.data_mut().with_mut(|data| data.method = option.value.clone() );
                },
                body: payment_items.clone()
            }
            div {
                h3 { class: "form__label", "Recipient" }
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
                                "Debug LLC"
                            }
                            p { class: "card-send2__info__description",
                                "Last Paid on Aug 26, 2024"
                            }
                        }
                    }
                    Button {
                        text: "Edit",
                        size: ElementSize::Small,
                        variant: ButtonVariant::Tertiary,
                        status: None,
                        on_click: move |_| {
                            onboarding_step.set(BillStep::Recipient);
                        }
                    }
                }
            }
            div {
                h3 { class: "form__label", "Recipient Bank Details" }
                div { class: "recipient__info",
                    div { class: "recipient__card",
                        Icon { icon : UploadExport, height : 20, width : 20, fill :
                        "var(--state-primary-active)" }
                        div { class: "card-send2__info",
                            h5 { class: "card-send2__info__title",
                                "Bancolombia"
                            }
                            p { class: "card-send2__info__description",
                                "Account no. *6789 Routing *1010"
                            }
                        }
                    }
                    Button {
                        text: "Edit",
                        size: ElementSize::Small,
                        variant: ButtonVariant::Tertiary,
                        status: None,
                        on_click: move |_| {
                            onboarding_step.set(BillStep::Recipient);
                        }
                    }
                }
            }
            Input {
                message: bill_transaction.get_data().invoice_number,
                placeholder: "".to_string(),
                size: ElementSize::Small,
                label: "Invoice number (optional)",
                error: None,
                maxlength: 150,
                on_input: move |event: Event<FormData>| {
                    bill_transaction.data_mut().with_mut(|data| data.invoice_number = event.value() );
                },
                on_keypress: move |_| {},
                on_click: move |_| {},on_focus: move |_| {}, on_blur: move |_| {}
            }
            div { class: "row",
                Input {
                    message: bill_transaction.get_data().invoice_start_at,
                    placeholder: "".to_string(),
                    size: ElementSize::Small,
                    label: "Invoice date",
                    itype: InputType::Date,
                    error: None,
                    maxlength: 150,
                    on_input: move |event: Event<FormData>| {
                        bill_transaction.data_mut().with_mut(|data| data.invoice_start_at = event.value() );
                    },
                    on_keypress: move |_| {},
                    on_click: move |_| {},on_focus: move |_| {}, on_blur: move |_| {}
                }
                Input {
                    message: bill_transaction.get_data().invoice_end_at,
                    placeholder: "".to_string(),
                    size: ElementSize::Small,
                    label: "Due date",
                    itype: InputType::Date,
                    error: None,
                    maxlength: 150,
                    on_input: move |event: Event<FormData>| {
                        bill_transaction.data_mut().with_mut(|data| data.invoice_end_at = event.value() );
                    },
                    on_keypress: move |_| {},
                    on_click: move |_| {},on_focus: move |_| {}, on_blur: move |_| {}
                }
            }
            div { class: "row",
                Button {
                    text: "Back",
                    size: ElementSize::Small,
                    left_icon: rsx! {
                        Icon { icon: ChevronLeft, height: 20, width: 20, fill: "var(--base-fill-5)" }
                    },
                    status: None,
                    on_click: move |_| {
                        onboarding_step.set(BillStep::Recipient);
                    }
                }
                Button {
                    text: "Continue",
                    size: ElementSize::Small,
                    right_icon: rsx! {
                        Icon { icon: ChevronRight, height: 20, width: 20, fill: "var(--base-fill-5)" }
                    },
                    status: None,
                    on_click: move |_| {
                        onboarding_step.set(BillStep::Amount);
                    }
                }
            }
        }
    )
}
