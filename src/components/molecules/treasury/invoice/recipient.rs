use dioxus::prelude::*;

use crate::{
    components::atoms::{
        autocomplete::AutocompleteItem, button::Variant, dropdown::ElementSize, AddPlus,
        Autocomplete, AutocompleteItemButton, Button, ChevronRight, Icon,
    },
    hooks::use_recipients::use_recipients,
    pages::invoice::InvoiceStep,
};

#[derive(Debug, Clone, Default)]
pub struct RecipientValueKey {
    pub id: u16,
    pub alias: String,
    pub category: String,
    pub name: Option<String>,
}

impl RecipientValueKey {
    pub fn new(id: u16, alias: String, category: String, name: Option<String>) -> Self {
        Self {
            id,
            alias,
            category,
            name,
        }
    }
}

#[component]
pub fn RecipientForm() -> Element {
    let mut onboarding_step = use_context::<Signal<InvoiceStep>>();

    let mut dropdown_recipient_value = use_signal::<Option<AutocompleteItem>>(|| None);
    let mut filter_name = use_signal(|| String::new());
    let mut recipients = use_recipients();

    rsx!(
        section { class: "bill__form",
            h3 { class: "send__form__title", "Create Invoice for" }
            Autocomplete {
                value: dropdown_recipient_value(),
                placeholder: "",
                label: "Name",
                reverse: false,
                size: ElementSize::Small,
                default: None,
                on_change: move |event: usize| {
                    let binding = recipients.get_recipients_by_filters(Some(&filter_name()), None);
                    let x = binding.get(event);

                    if let Some(recipient) = x {
                        dropdown_recipient_value.set(Some(
                            AutocompleteItem { key: recipient.id.to_string(), value: recipient.alias.clone() }
                        ));
                        if let Ok(()) = recipients.set_recipient(recipient.id) {
                            onboarding_step.set(InvoiceStep::Details);
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
                add_element: rsx!{
                    Button {
                        class: "autocomplete__item-cta-add",
                        text: "Add Recipient",
                        size: ElementSize::Medium,
                        variant: Variant::Tertiary,
                        status: None,
                        left_icon: rsx! {
                            Icon { icon: AddPlus, height: 24, width: 24, fill: "var(--base-fill-5)" }
                        },
                        on_click: move |_| {

                        }
                    }
                }
            }
            div { class: "row",
                Button {
                    text: "Invoice Details",
                    size: ElementSize::Small,
                    status: None,
                    right_icon: rsx! {
                        Icon { icon: ChevronRight, height: 20, width: 20, fill: "var(--base-fill-5)" }
                    },
                    on_click: move |_| {
                        onboarding_step.set(InvoiceStep::Details);
                    }
                }
            }
        }
    )
}
