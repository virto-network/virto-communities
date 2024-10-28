use dioxus::prelude::*;

use crate::{
    components::atoms::{
        dropdown::{DropdownItem, ElementSize},
        icon_button::Variant as IconButtonVariant,
        Attach, AutocompleteItemButton, Button, ChevronLeft, ChevronRight, Close, Dropdown,
        FileLine, Icon, IconButton, TextareaInput,
    },
    hooks::{use_attach::AttachFile, use_send::use_send},
};

use super::recipient::ControlProps;

#[component]
pub fn InfoForm(props: ControlProps) -> Element {
    let mut dropdown_gl_code_value = use_signal::<Option<DropdownItem>>(|| None);
    let mut gl_code_items = vec![];
    let mut send_transaction = use_send();

    let list_gl_code_items_mock = vec![DropdownItem {
        key: "120".to_string(),
        value: "120 - Accounts Payable".to_string(),
    }];
    for account in list_gl_code_items_mock.iter() {
        gl_code_items.push(rsx!(AutocompleteItemButton {
            title: account.value.clone(),
        }))
    }

    rsx!(
        section { class: "send__form",
            h3 {class: "send__form__title", "Additional Info"}
            TextareaInput {
                value: send_transaction.get_data().memo,
                placeholder: "".to_string(),
                label: "Memo".to_string(),
                help: "Sent to the recipient (e.g. For further credit, For benefit of, or a custom message)".to_string(),
                on_input: move |event: Event<FormData>| {
                    send_transaction.data_mut().with_mut(|data| data.memo = event.value().clone() );
                },
                on_keypress: move |_| {},
                on_click: move |_| {}
            }
            div {
                h2 { class: "form__subtitle", "Notes and Attachments" }
                h3 { class: "form__label", "Only visible to people with access to your Tunja Community Initiative" }
            }
            Dropdown {
                class: "header__wallet".to_string(),
                value: dropdown_gl_code_value(),
                placeholder: "Accounts Payable",
                label: "GL Code",
                size: ElementSize::Small,
                default: None,
                on_change: move |event: usize| {
                    let option = &list_gl_code_items_mock[event];
                    dropdown_gl_code_value.set(Some(option.clone()));
                    send_transaction.data_mut().with_mut(|data| data.payment_via = option.value.clone() );
                },
                body: gl_code_items.clone()
            }
            TextareaInput {
                value: send_transaction.get_data().notes,
                placeholder: "".to_string(),
                label: "Notes".to_string(),
                on_input: move |event: Event<FormData>| {
                    send_transaction.data_mut().with_mut(|data| data.notes = event.value().clone() );
                },
                on_keypress: move |_| {},
                on_click: move |_| {}
            }
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
            Attach {
                cta_text: "Upload Attachment".to_string(),
                supported_types: vec![String::from("image/png"), String::from("image/png")],
                on_change: move |_: AttachFile| {
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
