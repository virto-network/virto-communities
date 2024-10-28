use dioxus::prelude::*;

use crate::{
    components::atoms::{
        dropdown::{DropdownItem, ElementSize},
        AutocompleteItemButton, Button, ChevronLeft, ChevronRight, Dropdown, Icon, Input,
        RadioButton,
    },
    hooks::{
        use_recipient::{use_recipient, RecipientType},
        use_recipients::use_recipients,
    },
};

use super::recipient::ControlProps;

#[component]
pub fn RecipientForm(props: ControlProps) -> Element {
    let mut dropdown_country_value = use_signal::<Option<DropdownItem>>(|| None);
    let mut country_items = vec![];
    let mut dropdown_state_value = use_signal::<Option<DropdownItem>>(|| None);
    let mut state_items = vec![];
    let mut recipient = use_recipient();
    let mut recipients = use_recipients();

    let list_country_items_mock = vec![DropdownItem {
        key: "Colombia".to_string(),
        value: "Colombia".to_string(),
    }];
    for account in list_country_items_mock.iter() {
        country_items.push(rsx!(AutocompleteItemButton {
            title: account.value.clone(),
        }))
    }

    let list_state_items_mock = vec![
        DropdownItem {
            key: "Boyaca".to_string(),
            value: "Boyacá".to_string(),
        },
        DropdownItem {
            key: "Cundinamarca".to_string(),
            value: "Cundinamarca".to_string(),
        },
    ];
    for account in list_state_items_mock.iter() {
        state_items.push(rsx!(AutocompleteItemButton {
            title: account.value.clone(),
        }))
    }

    rsx!(
        section { class: "send__form",
            h3 {class: "send__form__title", "Recipient Details"}
            div { class: "form__inputs__container__cta",
                RadioButton {
                    title: "Person".to_string(),
                    name: "Person",
                    checked: matches!(recipient.get_recipient().details, RecipientType::Person),
                    on_change: move |_| {
                        recipient.recipient_mut().with_mut(|recipient| recipient.details = RecipientType::Person);
                    }
                }
                RadioButton {
                    title: "Business".to_string(),
                    name: "Business",
                    checked: matches!(recipient.get_recipient().details, RecipientType::Business),
                    on_change: move |_| {
                        recipient.recipient_mut().with_mut(|recipient| recipient.details = RecipientType::Business);
                    }
                }
                RadioButton {
                    title: "DAO".to_string(),
                    name: "DAO",
                    checked: matches!(recipient.get_recipient().details, RecipientType::Dao),
                    on_change: move |_| {
                        recipient.recipient_mut().with_mut(|recipient| recipient.details = RecipientType::Dao);
                    }
                }
                RadioButton {
                    title: "Initiative".to_string(),
                    name: "Initiative",
                    checked: matches!(recipient.get_recipient().details, RecipientType::Initiative),
                    on_change: move |_| {
                        recipient.recipient_mut().with_mut(|recipient| recipient.details = RecipientType::Initiative);
                    }
                }
            }
            Input {
                message: recipient.get_recipient().name,
                placeholder: "".to_string(),
                size: ElementSize::Small,
                label: "Full Name",
                error: None,
                maxlength: 150,
                on_input: move |event: Event<FormData>| {
                    recipient.recipient_mut().with_mut(|recipient| recipient.name = event.value());
                },
                on_keypress: move |_| {},
                on_click: move |_| {},on_focus: move |_| {}, on_blur: move |_| {}
            }
            Input {
                message: recipient.get_recipient().email,
                placeholder: "".to_string(),
                size: ElementSize::Small,
                label: "Email",
                error: None,
                maxlength: 150,
                on_input: move |event: Event<FormData>| {
                    recipient.recipient_mut().with_mut(|recipient| recipient.email = event.value());
                },
                on_keypress: move |_| {},
                on_click: move |_| {},on_focus: move |_| {}, on_blur: move |_| {}
            }
            Input {
                message: recipient.get_recipient().alias,
                placeholder: "".to_string(),
                size: ElementSize::Small,
                label: "Alias",
                error: None,
                maxlength: 150,
                on_input: move |event: Event<FormData>| {
                    recipient.recipient_mut().with_mut(|recipient| recipient.alias = event.value());
                },
                on_keypress: move |_| {},
                on_click: move |_| {},on_focus: move |_| {}, on_blur: move |_| {}
            }
            hr {class: "divider"}
            Dropdown {
                class: "header__wallet".to_string(),
                value: dropdown_country_value(),
                placeholder: "",
                label: "Country",
                size: ElementSize::Small,
                default: None,
                on_change: move |event: usize| {
                    let option = &list_country_items_mock.get(event);
                    dropdown_country_value.set(option.cloned());

                    if let Some(opt) = option {
                        recipient.recipient_mut().with_mut(|recipient| recipient.country = opt.value.clone());
                    };
                },
                body: country_items.clone()
            }
            Input {
                message: recipient.get_recipient().address,
                placeholder: "".to_string(),
                size: ElementSize::Small,
                label: "Recipient Legal Address",
                error: None,
                maxlength: 150,
                on_input: move |event: Event<FormData>| {
                    recipient.recipient_mut().with_mut(|recipient| recipient.address = event.value());
                },
                on_keypress: move |_| {},
                on_click: move |_| {},on_focus: move |_| {}, on_blur: move |_| {}
            }
            Input {
                message: recipient.get_recipient().address_complement,
                placeholder: "".to_string(),
                size: ElementSize::Small,
                label: "Recipient Legal Address",
                error: None,
                maxlength: 150,
                on_input: move |event: Event<FormData>| {
                    recipient.recipient_mut().with_mut(|recipient| recipient.address_complement = event.value());
                },
                on_keypress: move |_| {},
                on_click: move |_| {},on_focus: move |_| {}, on_blur: move |_| {}
            }
            Input {
                message: recipient.get_recipient().address_context,
                placeholder: "".to_string(),
                size: ElementSize::Small,
                label: "Apartment, suite, floor (optional)",
                error: None,
                maxlength: 150,
                on_input: move |event: Event<FormData>| {
                    recipient.recipient_mut().with_mut(|recipient| recipient.address_context = event.value());
                },
                on_keypress: move |_| {},
                on_click: move |_| {},on_focus: move |_| {}, on_blur: move |_| {}
            }
            div { class: "row",
                Input {
                    message: recipient.get_recipient().city,
                    placeholder: "".to_string(),
                    size: ElementSize::Small,
                    label: "City",
                    error: None,
                    maxlength: 150,
                    on_input: move |event: Event<FormData>| {
                        recipient.recipient_mut().with_mut(|recipient| recipient.city = event.value());
                    },
                    on_keypress: move |_| {},
                    on_click: move |_| {},on_focus: move |_| {}, on_blur: move |_| {}
                }
                Dropdown {
                    class: "header__wallet".to_string(),
                    value: dropdown_state_value(),
                    placeholder: "",
                    label: "State",
                    size: ElementSize::Small,
                    default: None,
                    on_change: move |event: usize| {
                        let option = &list_state_items_mock.get(event);
                        dropdown_state_value.set(option.cloned());

                        if let Some(opt) = option {
                            recipient.recipient_mut().with_mut(|recipient| recipient.state = opt.value.clone());
                        };
                    },
                    body: state_items.clone()
                }
            }
            Input {
                message: recipient.get_recipient().zip,
                placeholder: "".to_string(),
                size: ElementSize::Small,
                label: "Postal / ZIP code",
                error: None,
                maxlength: 150,
                on_input: move |event: Event<FormData>| {
                    recipient.recipient_mut().with_mut(|recipient| recipient.zip = event.value());
                },
                on_keypress: move |_| {},
                on_click: move |_| {},on_focus: move |_| {}, on_blur: move |_| {}
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
                        recipients.push_recipient(recipient.get_recipient());
                        props.on_next.call(());
                    }
                }
            }
        }
    )
}
