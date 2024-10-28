use dioxus::prelude::*;

use crate::{
    components::atoms::{
        autocomplete::AutocompleteItem, button::Variant, dropdown::ElementSize, AddPlus,
        Autocomplete, AutocompleteItemButton, Button, FileDropArea, Icon,
    },
    hooks::use_recipients::use_recipients,
};

#[derive(PartialEq, Props, Clone)]
pub struct RecipientSelectProps {
    on_add: EventHandler,
    on_next: EventHandler,
}

#[component]
pub fn RecipientSelect(props: RecipientSelectProps) -> Element {
    let mut dropdown_recipient_value = use_signal::<Option<AutocompleteItem>>(|| None);
    let mut filter_name = use_signal(|| String::new());
    let mut recipients = use_recipients();

    rsx!(
        section { class: "send__form",
            h3 {class: "send__form__title", "Recipient"}
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
                            props.on_next.call(())
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
                            props.on_add.call(())
                        }
                    }
                }
            }
            FileDropArea {
                label: "Pay a Bill"
            }
        }
    )
}
