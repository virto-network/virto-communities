use dioxus::prelude::*;

use crate::{
    components::atoms::{
        dropdown::{DropdownItem, ElementSize},
        input_tags::InputTagsEvent,
        Button, ChevronLeft, ChevronRight, Dropdown, Icon, Input, InputTags, TextareaInput,
    },
    pages::bill::BillStep,
};

#[component]
pub fn InfoForm() -> Element {
    let mut dropdown_value = use_signal::<Option<DropdownItem>>(|| None);
    let mut items = vec![];
    let mut onboarding_step = use_context::<Signal<BillStep>>();
    rsx!(
        section { class: "bill__form",
            h3 {class: "send__form__title", "Anything you’d like to add?"}
            Dropdown {
                class: "header__wallet".to_string(),
                value: dropdown_value(),
                placeholder: "Select",
                label: "GL Code",
                size: ElementSize::Small,
                default: None,
                on_change: move |event: usize| {

                },
                body: items.clone()
            }
            TextareaInput {
                value: "From Tunja Community, Incorporated via Virto.network for invoice INV-902".to_string(),
                placeholder: "".to_string(),
                label: "Memo for Debug LLC (optional)",
                help: "Sent to recipients bank",
                on_input: move |event: Event<FormData>| {
                },
                on_keypress: move |_| {},
                on_click: move |_| {}
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
            InputTags {
                message: "",
                size: ElementSize::Small,
                placeholder: "",
                label: "Email receipt to (optional)",
                error: None,
                maxlength: 5,
                on_input: move |event: InputTagsEvent| {

                },
                on_keypress: move |_| {},
                on_click: move |_| {}
            }
            div { class: "row",
                Button {
                    text: "Amount",
                    size: ElementSize::Small,
                    left_icon: rsx! {
                        Icon { icon: ChevronLeft, height: 20, width: 20, fill: "var(--base-fill-5)" }
                    },
                    status: None,
                    on_click: move |_| {
                        onboarding_step.set(BillStep::Amount);
                    }
                }
                Button {
                    text: "Review & Pay",
                    size: ElementSize::Small,
                    right_icon: rsx! {
                        Icon { icon: ChevronRight, height: 20, width: 20, fill: "var(--base-fill-5)" }
                    },
                    status: None,
                    on_click: move |_| {
                        onboarding_step.set(BillStep::Review);
                    }
                }
            }
        }
    )
}
