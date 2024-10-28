use std::{ops::Deref, rc::Rc};

use dioxus::prelude::*;

use crate::{
    components::{
        atoms::{
            autocomplete::AutocompleteItem,
            dropdown::{DropdownItem, ElementSize},
            input::InputType,
            Autocomplete, AutocompleteItemButton, Button, Checkbox, ChevronLeft, ChevronRight,
            Dropdown, Icon, Input, RadioButton, Switch, TextareaInput, WalletLine,
        },
        molecules::send::amount::DropdownAccountItem,
    },
    pages::invoice::InvoiceStep,
};

pub enum RepeatPayment {
    Weekly,
    MonthlyNumber,
    MonthlyDay,
    Custom,
}

#[component]
pub fn PaymentMethodForm() -> Element {
    let mut dropdown_value = use_signal::<Option<DropdownItem>>(|| None);
    let mut dropdown_value_every = use_signal::<Option<DropdownItem>>(|| None);
    let mut is_repeat_invoice = use_signal::<bool>(|| false);
    let mut is_accept_credit_cards = use_signal::<bool>(|| false);
    let mut items = vec![];
    let mut items2 = vec![];
    let mut items_every = vec![];

    let mut dropdown_value2 = use_signal::<Option<AutocompleteItem>>(|| None);
    let mut filtered_items = use_signal::<Vec<AutocompleteItem>>(|| vec![]);
    let mut onboarding_step = use_context::<Signal<InvoiceStep>>();

    let list_items_mock = Rc::new(vec![
        AutocompleteItem {
            key: "weekly".to_string(),
            value: "Weekly on Friday".to_string(),
        },
        AutocompleteItem {
            key: "monthly_number".to_string(),
            value: "Monthly on the 4th".to_string(),
        },
        AutocompleteItem {
            key: "monthly_day".to_string(),
            value: "Monthly on the 1st Friday".to_string(),
        },
        AutocompleteItem {
            key: "custom".to_string(),
            value: "Custom".to_string(),
        },
    ]);

    let list_items_mock2 = list_items_mock.clone();
    let list_items_mock3 = list_items_mock.clone();

    for account in <Vec<AutocompleteItem> as Clone>::clone(&list_items_mock).into_iter() {
        items.push(rsx!(AutocompleteItemButton {
            title: account.value,
        }))
    }

    filtered_items.set(list_items_mock.to_vec());

    let mut account_value = use_signal::<Option<DropdownAccountItem>>(|| None);

    let list_items2_mock = Rc::new(vec![
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

    let list_items2_mock2 = list_items2_mock.clone();

    for account in <Vec<DropdownAccountItem> as Clone>::clone(&list_items2_mock).into_iter() {
        let description = format!("{} / *{}", account.balance, account.id);
        items2.push(rsx!(
            div { class: "card-send2__info",
                h5 { class: "card-send3__info__title",
                    {account.name}
                }
                p { class: "card-send3__info__description",
                    {description}
                }
            }
        ));
    }

    let list_items3_mock = Rc::new(vec![
        DropdownItem {
            key: "month".to_string(),
            value: "Month".to_string(),
        },
        DropdownItem {
            key: "week".to_string(),
            value: "Week".to_string(),
        },
    ]);

    for account in <Vec<DropdownItem> as Clone>::clone(&list_items3_mock).into_iter() {
        items_every.push(rsx!(AutocompleteItemButton {
            title: account.value,
        }))
    }
    rsx!(
        section { class: "bill__form",
            h3 {class: "send__form__title", "Payment Details"}
            div { class: "row",
                Input {
                    message: "".to_string(),
                    placeholder: "".to_string(),
                    size: ElementSize::Small,
                    label: "Invoice date",
                    itype: InputType::Date,
                    error: None,
                    maxlength: 150,

                    on_input: move |event: Event<FormData>| {
                    },
                    on_keypress: move |_| {},
                    on_click: move |_| {},on_focus: move |_| {}, on_blur: move |_| {}
                }
                Dropdown {
                    class: "header__wallet".to_string(),
                    value: dropdown_value(),
                    placeholder: "Select",
                    label: "Due Date",
                    size: ElementSize::Small,
                    default: None,
                    on_change: move |event: usize| {

                    },
                    body: items.clone()
                }
            }
            Switch {
                id: "repeat-invoice",
                name: "repeat-invoice",
                label: "Repeat this invoice",
                checked: is_repeat_invoice(),
                on_change: move |_| {is_repeat_invoice.toggle()}
            }
            if is_repeat_invoice() {
                div { class: "form__repeat",
                    Autocomplete {
                        value: dropdown_value2(),
                        placeholder: "",
                        label: "Repeat",
                        size: ElementSize::Medium,
                        default: None,
                        on_change: move |event: usize| {
                            log::info!("change event {}", event);
                            let x = &list_items_mock2[event];
                            log::info!("change event {:?}", x);
                            dropdown_value2.set(Some(x.clone()))
                        },
                        on_input: move |event: FormEvent| {
                            if event.value().is_empty() {
                                filtered_items.set(list_items_mock.to_vec());
                                return;
                            }

                            let list_items_mock = <Vec<AutocompleteItem> as Clone>::clone(&list_items_mock3)
                            .into_iter()
                            .filter(|item| item.value.to_lowercase().contains(&event.value().to_lowercase()))
                            .collect::<Vec<AutocompleteItem>>();

                            filtered_items.set(list_items_mock);
                        },
                        body: items.clone()
                    }
                    if let Some(element) = dropdown_value2() {
                        if element.key.deref() == "custom" {
                            div { class: "repeat-custom-payment",
                                div {
                                    div { class: "repeat-custom-payment--row",
                                        span { class: "input__label", "Repeat every"}
                                    }
                                    div {class: "row repeat-custom-payment--row",
                                        Dropdown {
                                            class: "header__wallet".to_string(),
                                            value: dropdown_value(),
                                            placeholder: "Select",
                                            label: "",
                                            size: ElementSize::Small,
                                            default: None,
                                            on_change: move |event: usize| {

                                            },
                                            body: items.clone()
                                        }
                                        Dropdown {
                                            class: "header__wallet".to_string(),
                                            value: dropdown_value_every(),
                                            placeholder: "Select",
                                            label: "",
                                            size: ElementSize::Small,
                                            default: None,
                                            on_change: move |event: usize| {
                                                log::info!("change event {}", event);
                                                let x = &list_items3_mock[event];
                                                log::info!("change event {:?}", x);
                                                dropdown_value_every.set(Some(x.clone()))
                                            },
                                            body: items_every.clone()
                                        }
                                    }
                                }
                                div {
                                    div { class: "repeat-custom-payment--row",
                                        span { class: "input__label", "Repeat on"}
                                    }
                                    if let Some(e) = dropdown_value_every() {
                                        if e.key.deref() == "month" {
                                            div {class: "row repeat-custom-payment--row",
                                                RadioButton {
                                                    name: "Day",
                                                    title: "Day",
                                                    checked: false,
                                                    on_change: move |_| {}
                                                }
                                                Dropdown {
                                                    class: "header__wallet".to_string(),
                                                    value: dropdown_value(),
                                                    placeholder: "Select",
                                                    label: "",
                                                    size: ElementSize::Small,
                                                    default: None,
                                                    on_change: move |event: usize| {

                                                    },
                                                    body: items.clone()
                                                }
                                            }
                                            div {class: "row repeat-custom-payment--row",
                                                RadioButton {
                                                    name: "The",
                                                    title: "The",
                                                    checked: false,
                                                    on_change: move |_| {}
                                                }
                                                div { class: "row",
                                                    Dropdown {
                                                        class: "header__wallet".to_string(),
                                                        value: dropdown_value(),
                                                        placeholder: "Select",
                                                        label: "",
                                                        size: ElementSize::Small,
                                                        default: None,
                                                        on_change: move |event: usize| {

                                                        },
                                                        body: items.clone()
                                                    }
                                                    Dropdown {
                                                        class: "header__wallet".to_string(),
                                                        value: dropdown_value(),
                                                        placeholder: "Select",
                                                        label: "",
                                                        size: ElementSize::Small,
                                                        default: None,
                                                        on_change: move |event: usize| {

                                                        },
                                                        body: items.clone()
                                                    }
                                                }
                                            }
                                            div {class: "row repeat-custom-payment--row",
                                                RadioButton {
                                                    name: "Never",
                                                    title: "Never",
                                                    checked: false,
                                                    on_change: move |_| {}
                                                }
                                            }
                                        }
                                        if e.key.deref() == "week" {
                                            div {class: "row repeat-custom-payment--row",
                                                RadioButton {
                                                    name: "Day",
                                                    title: "Day",
                                                    checked: false,
                                                    on_change: move |_| {}
                                                }
                                            }
                                        }
                                    }
                                }

                            }
                        }
                        div { class: "repeat-custom-payment",
                            div { class: "repeat-custom-payment--row",
                                span { class: "input__label", "Ends"}
                            }
                            div {class: "row ",
                                RadioButton {
                                    name: "On",
                                    title: "On",
                                    checked: false,
                                    on_change: move |_| {}
                                }
                                Input {
                                    message: "".to_string(),
                                    placeholder: "".to_string(),
                                    size: ElementSize::Small,
                                    label: "",
                                    itype: InputType::Date,
                                    error: None,
                                    maxlength: 150,

                                    on_input: move |event: Event<FormData>| {
                                    },
                                    on_keypress: move |_| {},
                                    on_click: move |_| {},on_focus: move |_| {}, on_blur: move |_| {}
                                }
                            }
                            div {class: "row",
                                RadioButton {
                                    name: "After",
                                    title: "After",
                                    checked: false,
                                    on_change: move |_| {}
                                }
                                Input {
                                    message: "".to_string(),
                                    placeholder: "".to_string(),
                                    size: ElementSize::Small,
                                    label: "",
                                    itype: InputType::Date,
                                    error: None,
                                    maxlength: 150,

                                    on_input: move |event: Event<FormData>| {
                                    },
                                    on_keypress: move |_| {},
                                    on_click: move |_| {},on_focus: move |_| {}, on_blur: move |_| {}
                                }
                            }
                            div {class: "row",
                                RadioButton {
                                    name: "Never",
                                    title: "Never",
                                    checked: false,
                                    on_change: move |_| {}
                                }
                            }
                        }
                    }
                }
            }
            TextareaInput {
                value: "".to_string(),
                placeholder: "".to_string(),
                label: "Payer memo (optional)",
                on_input: move |event: Event<FormData>| {
                },
                on_keypress: move |_| {},
                on_click: move |_| {}
            }
            div { class: "information",
                span { class: "information__title",
                    "Payment Methods"
                }
                p { class: "information__description",
                    "Accepting payments via settings preset: Kusama Network, Bancolombia, Nequi and Daviplata Check Payment Settings Presets and Fees."
                }
            }
            Checkbox {
                id: "accept-credit-cards",
                name: "accept-credit-cards",
                label: "Accept Credit Cards",
                checked: is_accept_credit_cards(),
                on_change: move |_| {is_accept_credit_cards.toggle()}
            }
            div { class: "information",
                span { class: "information__title",
                    "Internal set up"
                }
                p { class: "information__description",
                    "Only visible to members in your organization."
                }
            }
            Dropdown {
                class: "account-selector".to_string(),
                value: account_value().map(|account|DropdownItem { key: account.id, value: account.name }),
                value_help: account_value().map(|account|format!("{} / *{}", account.balance, account.id)),
                placeholder: "Select Account",
                label: "Destination account",
                size: ElementSize::Small,
                default: None,
                help: "Payments are routed through a secure account to keep your details private.",
                left_icon: rsx!(
                    Icon { icon : WalletLine, height : 24, width : 24, fill : "var(--state-primary-active)" }
                ),
                on_change: move |event: usize| {
                    log::info!("change event {}", event);
                    let x = &list_items2_mock2[event];
                    log::info!("change event {:?}", x);
                    account_value.set(Some(x.clone()))
                },
                body: items2.clone()
            }
            TextareaInput {
                value: "".to_string(),
                placeholder: "".to_string(),
                label: "Internal note (optional)",
                on_input: move |event: Event<FormData>| {
                },
                on_keypress: move |_| {},
                on_click: move |_| {}
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
                        onboarding_step.set(InvoiceStep::Details);
                    }
                }
                Button {
                    text: "Review",
                    size: ElementSize::Small,
                    right_icon: rsx! {
                        Icon { icon: ChevronRight, height: 20, width: 20, fill: "var(--base-fill-5)" }
                    },
                    status: None,
                    on_click: move |_| {
                        onboarding_step.set(InvoiceStep::Review);
                    }
                }
            }
        }
    )
}
