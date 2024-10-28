use std::rc::Rc;

use dioxus::prelude::*;

use crate::{
    components::atoms::{
        dropdown::{DropdownItem, ElementSize},
        input::InputType,
        AutocompleteItemButton, Button, ChevronLeft, ChevronRight, Dropdown, Icon, Input, Switch,
        WalletLine,
    },
    hooks::use_send::use_send,
};

use super::recipient::ControlProps;
#[derive(PartialEq, Debug, Clone, Default)]
pub struct DropdownAccountItem {
    pub id: String,
    pub name: String,
    pub balance: String,
}
#[component]
pub fn AmountForm(props: ControlProps) -> Element {
    let mut account_value = use_signal::<Option<DropdownAccountItem>>(|| None);
    let mut dropdown_currency_value = use_signal::<Option<DropdownItem>>(|| None);
    let mut currency_items = vec![];
    let mut account_from_items = vec![];
    let mut is_repeat_payment = use_signal(|| false);
    let mut send_transaction = use_send();

    let list_currency_items_mock = vec![DropdownItem {
        key: "usd".to_string(),
        value: "USD".to_string(),
    }];
    for account in list_currency_items_mock.iter() {
        currency_items.push(rsx!(AutocompleteItemButton {
            title: account.value.clone(),
        }))
    }

    let list_account_from_items_mock = Rc::new(vec![
        DropdownAccountItem {
            id: String::from("10298309128"),
            name: String::from("Personal"),
            balance: String::from("2000"),
        },
        DropdownAccountItem {
            id: String::from("10298309129"),
            name: String::from("Business"),
            balance: String::from("15000"),
        },
        DropdownAccountItem {
            id: String::from("10298309130"),
            name: String::from("Savings"),
            balance: String::from("5000"),
        },
        DropdownAccountItem {
            id: String::from("10298309131"),
            name: String::from("Investment"),
            balance: String::from("25000"),
        },
    ]);

    for account in list_account_from_items_mock.iter() {
        let description = format!("{} / *{}", account.balance, account.id);
        account_from_items.push(rsx!(
            div { class: "card-send2__info",
                h5 { class: "card-send3__info__title",
                    "{account.name}"
                }
                p { class: "card-send3__info__description",
                    {description}
                }
            }
        ));
    }

    rsx!(
        section { class: "send__form",
            h3 {class: "send__form__title", "Amount & Source"}
            div { class: "row",
                Dropdown {
                    class: "header__wallet".to_string(),
                    value: dropdown_currency_value(),
                    placeholder: "Select",
                    label: "Currency",
                    size: ElementSize::Small,
                    default: None,
                    on_change: move |event: usize| {
                        let option = &list_currency_items_mock[event];
                        dropdown_currency_value.set(Some(option.clone()));
                        send_transaction.data_mut().with_mut(|data| data.currency = option.value.clone() );
                    },
                    body: currency_items.clone()
                }
                Input {
                    message: send_transaction.get_data().amount,
                    placeholder: "".to_string(),
                    size: ElementSize::Small,
                    label: "Recipient Gets",
                    error: None,
                    maxlength: 150,
                    on_input: move |event: Event<FormData>| {
                        send_transaction.data_mut().with_mut(|data| data.amount = event.value().parse::<u64>().unwrap_or(0) );
                    },
                    on_keypress: move |_| {},
                    on_click: move |_| {},on_focus: move |_| {}, on_blur: move |_| {}
                }
            }
            Dropdown {
                class: "account-selector".to_string(),
                value: account_value().map(|account|DropdownItem { key: account.id, value: account.name }),
                value_help: account_value().map(|account|format!("{} / *{}", account.balance, account.id)),
                placeholder: "Select Account",
                label: "Send From",
                size: ElementSize::Small,
                default: None,
                left_icon: rsx!(
                    Icon { icon : WalletLine, height : 24, width : 24, fill : "var(--state-primary-active)" }
                ),
                on_change: move |event: usize| {
                    let option = &list_account_from_items_mock[event];
                    account_value.set(Some(option.clone()));
                    send_transaction.data_mut().with_mut(|data| data.from = option.name.clone() );
                },
                body: account_from_items.clone()
            }
            Input {
                message: send_transaction.get_data().payment_at,
                placeholder: "".to_string(),
                size: ElementSize::Small,
                label: "Send on",
                error: None,
                itype: InputType::Date,
                maxlength: 150,
                on_input: move |event: Event<FormData>| {
                    send_transaction.data_mut().with_mut(|data| data.payment_at = event.value().clone() );
                },
                on_keypress: move |_| {},
                on_click: move |_| {},on_focus: move |_| {}, on_blur: move |_| {}
            }
            Switch {
                id: "repeat-payment",
                name: "repeat-payment",
                label: "Repeat this payment",
                checked: is_repeat_payment(),
                on_change: move |_| {is_repeat_payment.toggle()}
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
