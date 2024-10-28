use dioxus::prelude::*;

use crate::components::atoms::{
    dropdown::{DropdownItem, ElementSize},
    ArrowLeft, Button, ChevronLeft, ChevronRight, Dropdown, Icon, Input,
};

use crate::pages::invoice::InvoiceStep;

#[component]
pub fn AmountForm() -> Element {
    let mut dropdown_value = use_signal::<Option<DropdownItem>>(|| None);
    let mut items = vec![];
    let mut onboarding_step = use_context::<Signal<InvoiceStep>>();
    rsx!(
        section { class: "bill__form",
            h3 {class: "send__form__title", "Amount & Source"}
            div { class: "row",
                Dropdown {
                    class: "header__wallet".to_string(),
                    value: dropdown_value(),
                    placeholder: "Select",
                    label: "Currency",
                    size: ElementSize::Small,
                    default: None,
                    on_change: move |event: usize| {

                    },
                    body: items.clone()
                }
                Input {
                    message: "".to_string(),
                    placeholder: "".to_string(),
                    size: ElementSize::Small,
                    label: "Recipient Gets",
                    error: None,
                    maxlength: 150,

                    on_input: move |event: Event<FormData>| {
                    },
                    on_keypress: move |_| {},
                    on_click: move |_| {},on_focus: move |_| {}, on_blur: move |_| {}
                }
            }
            Dropdown {
                class: "header__wallet".to_string(),
                value: dropdown_value(),
                placeholder: "Select Account",
                label: "Send From",
                size: ElementSize::Small,
                default: None,
                left_icon: rsx!(
                    Icon { icon : ArrowLeft, height : 24, width : 24, fill : "var(--text-primary)" }
                ),
                on_change: move |event: usize| {

                },
                body: items.clone()
            }
            Input {
                message: "".to_string(),
                placeholder: "".to_string(),
                size: ElementSize::Small,
                label: "Send on",
                error: None,
                maxlength: 150,

                on_input: move |event: Event<FormData>| {
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
                        onboarding_step.set(InvoiceStep::PaymentMethod);
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
                        onboarding_step.set(InvoiceStep::Review);
                    }
                }
            }
        }
    )
}
