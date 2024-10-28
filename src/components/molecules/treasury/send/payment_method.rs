use dioxus::prelude::*;

use crate::{
    components::atoms::{
        avatar::Variant,
        dropdown::{DropdownItem, ElementSize},
        AutocompleteItemButton, Avatar, Button, ChevronLeft, ChevronRight, Dropdown, Icon, Input,
    },
    hooks::{use_recipients::use_recipients, use_send::use_send},
};

use super::recipient::ControlProps;

#[component]
pub fn PaymentMethodForm(props: ControlProps) -> Element {
    let mut dropdown_payment_value = use_signal::<Option<DropdownItem>>(|| None);
    let mut payment_items = vec![];
    let mut dropdown_id_value = use_signal::<Option<DropdownItem>>(|| None);
    let mut id_items = vec![];
    let mut dropdown_bank_value = use_signal::<Option<DropdownItem>>(|| None);
    let mut bank_items = vec![];
    let mut dropdown_account_type_value = use_signal::<Option<DropdownItem>>(|| None);
    let mut account_type_items = vec![];
    let recipients = use_recipients();
    let mut send_transaction = use_send();

    let list_payment_items_mock = vec![DropdownItem {
        key: "bank".to_string(),
        value: "Bank Transfer".to_string(),
    }];
    for account in list_payment_items_mock.iter() {
        payment_items.push(rsx!(AutocompleteItemButton {
            title: account.value.clone(),
        }))
    }

    let list_id_items_mock = vec![
        DropdownItem {
            key: "cc".to_string(),
            value: "CC".to_string(),
        },
        DropdownItem {
            key: "ti".to_string(),
            value: "TI".to_string(),
        },
    ];
    for account in list_id_items_mock.iter() {
        id_items.push(rsx!(AutocompleteItemButton {
            title: account.value.clone(),
        }))
    }

    let list_bank_items_mock = vec![
        DropdownItem {
            key: "bancolombia".to_string(),
            value: "Bancolombia".to_string(),
        },
        DropdownItem {
            key: "falabella".to_string(),
            value: "Falabella".to_string(),
        },
    ];
    for account in list_bank_items_mock.iter() {
        bank_items.push(rsx!(AutocompleteItemButton {
            title: account.value.clone(),
        }))
    }

    let list_bank_items_mock = vec![
        DropdownItem {
            key: "bancolombia".to_string(),
            value: "Bancolombia".to_string(),
        },
        DropdownItem {
            key: "falabella".to_string(),
            value: "Falabella".to_string(),
        },
    ];
    for account in list_bank_items_mock.iter() {
        bank_items.push(rsx!(AutocompleteItemButton {
            title: account.value.clone(),
        }))
    }

    let list_account_type_items_mock = vec![
        DropdownItem {
            key: "ahorros".to_string(),
            value: "Ahorros".to_string(),
        },
        DropdownItem {
            key: "corriente".to_string(),
            value: "Corriente".to_string(),
        },
    ];
    for account in <Vec<DropdownItem> as Clone>::clone(&list_account_type_items_mock).into_iter() {
        account_type_items.push(rsx!(AutocompleteItemButton {
            title: account.value,
        }))
    }

    rsx!(
        section { class: "send__form",
            "{send_transaction.get():?}"
            h3 {class: "send__form__title", "Payment Method"}
            Dropdown {
                class: "header__wallet".to_string(),
                value: dropdown_payment_value(),
                placeholder: "Bank Transfer",
                size: ElementSize::Small,
                default: None,
                help: Some("1 business day • 5% fee".to_string()),
                on_change: move |event: usize| {
                    let option = &list_payment_items_mock[event];
                    dropdown_payment_value.set(Some(option.clone()));
                    send_transaction.data_mut().with_mut(|data| data.method = option.value.clone() );
                },
                body: payment_items.clone()
            }
            div {
                h3 { class: "form__label", "Recipient" }
                div { class: "recipient__info recipient__info--details",
                    div { class: "recipient__card",
                        Avatar {
                            class: "recipient__info__avatar--details",
                            name: recipients.get_recipient().name,
                            size: 44,
                            uri: None,
                            variant: Variant::Round
                        }
                        div { class: "card-send2__info",
                            h5 { class: "card-send__info__title",
                                {recipients.get_recipient().alias}
                            }
                            p { class: "card-send__info__description",
                                {recipients.get_recipient().name}
                            }
                        }
                    }
                }
            }
            hr {class: "divider"}
            div {
                h3 { class: "form__label", "Recipient Bank Details" }
                div { class: "form__bank",
                    div { class: "row",
                        Dropdown {
                            class: "header__wallet".to_string(),
                            value: dropdown_id_value(),
                            placeholder: "Select...",
                            label: "ID Type",
                            size: ElementSize::Small,
                            default: None,
                            on_change: move |event: usize| {
                                let option = &list_id_items_mock[event];
                                dropdown_id_value.set(Some(option.clone()));
                                send_transaction.data_mut().with_mut(|data| data.id_type = option.value.clone() );
                            },
                            body: id_items.clone()
                        }
                        div { class: "xl-4",
                            Input {
                                message: send_transaction.get_data().id_number,
                                placeholder: "0000000000000".to_string(),
                                size: ElementSize::Small,
                                label: "ID Number",
                                error: None,
                                maxlength: 150,
                                on_input: move |event: Event<FormData>| {
                                    send_transaction.data_mut().with_mut(|data| data.id_number = event.value().clone() );
                                },
                                on_keypress: move |_| {},
                                on_click: move |_| {},on_focus: move |_| {}, on_blur: move |_| {}
                            }
                        }
                    }
                    div { class: "row",
                        Dropdown {
                            class: "header__wallet".to_string(),
                            value: dropdown_bank_value(),
                            placeholder: "Select".to_string(),
                            label: "Bank",
                            size: ElementSize::Small,
                            default: None,
                            on_change: move |event: usize| {
                                let option = &list_bank_items_mock[event];
                                dropdown_bank_value.set(Some(option.clone()));
                                send_transaction.data_mut().with_mut(|data| data.bank = option.value.clone() );
                            },
                            body: bank_items.clone()
                        }
                        Dropdown {
                            class: "header__wallet".to_string(),
                            value: dropdown_account_type_value(),
                            placeholder: "Select".to_string(),
                            label: "Account Type",
                            size: ElementSize::Small,
                            default: None,
                            on_change: move |event: usize| {
                                let option = &list_account_type_items_mock[event];
                                dropdown_account_type_value.set(Some(option.clone()));
                                send_transaction.data_mut().with_mut(|data| data.account_type = option.value.clone() );
                            },
                            body: account_type_items
                        }
                    }
                    Input {
                        message: send_transaction.get_data().account_number,
                        placeholder: "0000-0000-0000-0000".to_string(),
                        size: ElementSize::Small,
                        label: "Account Number",
                        error: None,
                        maxlength: 150,
                        on_input: move |event: Event<FormData>| {
                            send_transaction.data_mut().with_mut(|data| data.account_number = event.value().clone() );
                        },
                        on_keypress: move |_| {},
                        on_click: move |_| {},on_focus: move |_| {}, on_blur: move |_| {}
                    }
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
                        props.on_back.call(())
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
                        props.on_next.call(())
                    }
                }
            }
        }
    )
}
