use dioxus::prelude::*;

use crate::{
    components::atoms::{
        avatar::Variant, button::Variant as ButtonVariant, dropdown::ElementSize,
        icon_button::Variant as IconButtonVariant, AddPlus, Avatar, Button, ChevronLeft,
        ChevronRight, DollarMoneyCircle, Draggable, Icon, IconButton, Input, MinusCircle,
        PercentLine, Popover, QuestionLine, Sparkle,
    },
    hooks::use_invoice::InvoiceItem,
    pages::invoice::InvoiceStep,
};

#[component]
pub fn DetailsForm() -> Element {
    let mut onboarding_step = use_context::<Signal<InvoiceStep>>();
    let mut show_tax_input = use_signal(|| false);
    let mut list_items = use_signal::<Vec<InvoiceItem>>(|| vec![InvoiceItem::default()]);
    rsx!(
        section { class: "details__form",
            h3 {class: "send__form__title", "Invoice Details"}
            div {
                h3 { class: "form__label", "Creating invoice for" }
                div { class: "recipient__info recipient__info--details",
                    div { class: "recipient__card",
                        Avatar {
                            class: "recipient__info__avatar--details",
                            name: "DB",
                            size: 44,
                            uri: None,
                            variant: Variant::Round
                        }
                        div { class: "card-send2__info",
                            h5 { class: "card-send__info__title",
                                "Alias"
                            }
                            p { class: "card-send__info__description",
                                "Name"
                            }
                        }
                    }
                    Button {
                        class: "recipient__info__button--details",
                        text: "Edit",
                        size: ElementSize::Small,
                        variant: ButtonVariant::Tertiary,
                        status: None,
                        on_click: move |_| {
                            onboarding_step.set(InvoiceStep::Recipient);
                        }
                    }
                }
            }
            hr { class: "divider divider--table" }
            div { class: "row recipient__details--row",
                Input {
                    message: "".to_string(),
                    placeholder: "".to_string(),
                    size: ElementSize::Small,
                    label: "Invoice number",
                    error: None,
                    maxlength: 150,
                    right_text: rsx! {
                        Popover {
                            body: rsx!(
                                Icon { icon: Sparkle, height: 20, width: 20, fill: "var(--base-fill-5)" }
                            ),
                            text: "Automatically generated"
                        }
                    },
                    on_input: move |event: Event<FormData>| {
                    },
                    on_keypress: move |_| {},
                    on_click: move |_| {},
                    on_focus: move |_| {}, on_blur: move |_| {}
                }
                Input {
                    message: "".to_string(),
                    placeholder: "".to_string(),
                    size: ElementSize::Small,
                    label: "Purchase order number (optional)",
                    error: None,
                    maxlength: 150,
                    label_help: rsx!(
                        Popover {
                            body: rsx!(
                                Icon { icon: QuestionLine, height: 20, width: 20, fill: "var(--base-fill-5)" }
                            ),
                            text: "If provided by your customer, a purchase order is a
document sent by customers to sellers with the intention
to track and control the purchasing process."
                        }
                    ),
                    right_text: rsx! {
                        Popover {
                            body: rsx!(
                                Icon { icon: Sparkle, height: 20, width: 20, fill: "var(--base-fill-5)" }
                            ),
                            text: "Automatically generated"
                        }
                    },
                    on_input: move |event: Event<FormData>| {
                    },
                    on_keypress: move |_| {},
                    on_click: move |_| {},
                    on_focus: move |_| {}, on_blur: move |_| {}
                }
            }
            div {
                div { class: "row",
                    h3 { class: "form__label xl-5", "Item" }
                    h3 { class: "form__label xl-2", "Quantity" }
                    h3 { class: "form__label xl-3", "Price" }
                }
                div { class: "details__items",
                    for (index, item) in list_items().iter().enumerate() {
                        div { class: "row details--row",
                            IconButton {
                                variant: IconButtonVariant::Ghost,
                                class: "button--avatar button--drop",
                                body: rsx!(
                                    Icon { icon : Draggable, height : 13, width : 13, fill :
                                    "var(--base-fill-5)" }
                                ),
                                on_click: move |_| {}
                            }
                            div { class: "xl-5",
                                Input {
                                    message: "".to_string(),
                                    placeholder: "".to_string(),
                                    size: ElementSize::Small,
                                    label: "",
                                    error: None,
                                    maxlength: 150,
                                    on_input: move |event: Event<FormData>| {
                                    },
                                    on_keypress: move |_| {},
                                    on_click: move |_| {},on_focus: move |_| {}, on_blur: move |_| {}
                                }
                            }
                            div { class: "xl-2",
                                Input {
                                    message: "".to_string(),
                                    placeholder: "".to_string(),
                                    size: ElementSize::Small,
                                    label: "",
                                    error: None,
                                    maxlength: 150,
                                    on_input: move |event: Event<FormData>| {
                                    },
                                    on_keypress: move |_| {},
                                    on_click: move |_| {},on_focus: move |_| {}, on_blur: move |_| {}
                                }
                            }
                            div { class: "xl-3",
                                Input {
                                    message: "".to_string(),
                                    placeholder: "".to_string(),
                                    size: ElementSize::Small,
                                    label: "",
                                    error: None,
                                    maxlength: 150,
                                    left_text: rsx! {
                                        Icon { icon: DollarMoneyCircle, height: 20, width: 20, fill: "var(--base-fill-5)" }
                                    },
                                    on_input: move |event: Event<FormData>| {
                                    },
                                    on_keypress: move |_| {},
                                    on_click: move |_| {},
                                    on_focus: move |_| {}, on_blur: move |_| {}
                                }
                            }
                            IconButton {
                                class: "button--avatar button--drop",
                                variant: IconButtonVariant::Ghost,
                                body: rsx!(
                                    Icon { icon : MinusCircle, height : 13, width : 13, fill :
                                    "var(--base-fill-5)" }
                                ),
                                on_click: move |_| {
                                    list_items.remove(index);
                                }
                            }
                        }
                    }
                    if show_tax_input() {
                        div { class: "row details--row",
                            div { class: "xl-5",
                                span { class: "details__total__title", "Sales Tax (%)" }
                            }
                            div { class: "xl-5",
                                Input {
                                    message: "".to_string(),
                                    placeholder: "".to_string(),
                                    size: ElementSize::Small,
                                    label: "",
                                    error: None,
                                    maxlength: 150,
                                    right_text: rsx! {
                                        Icon { icon: PercentLine, height: 20, width: 20, fill: "var(--base-fill-5)" }
                                    },
                                    on_input: move |event: Event<FormData>| {
                                    },
                                    on_keypress: move |_| {},
                                    on_click: move |_| {},on_focus: move |_| {}, on_blur: move |_| {}
                                }
                            }
                        }
                    }

                    div { class: "row",
                        Button {
                            text: "Add Line Item",
                            size: ElementSize::Small,
                            variant: ButtonVariant::Secondary,
                            left_icon: rsx! {
                                Icon { icon: AddPlus, height: 20, width: 20, fill: "var(--base-fill-5)" }
                            },
                            status: None,
                            on_click: move |_| {
                                list_items.push(InvoiceItem::default());
                            }
                        }
                        if !show_tax_input() {
                            Button {
                                text: "Add Sales Tax",
                                size: ElementSize::Small,
                                variant: ButtonVariant::Tertiary,
                                status: None,
                                on_click: move |_| {
                                    show_tax_input.set(true);
                                }
                            }
                        }
                    }
                }
            }
            hr { class: "divider--item" }
            div { class: "details__items",
                if show_tax_input() {
                    div { class: "details__total",
                        span { class: "details__total__subtitle", "Subtotal" }
                        span { class: "details__total__value", "$0.00" }
                    }
                    div { class: "details__total",
                        span { class: "details__total__subtitle", "Tax (%)" }
                        span { class: "details__total__value", "$0.00" }
                    }
                }
                div { class: "details__total",
                    span { class: "details__total__title", "Total" }
                    span { class: "details__total__value", "$0.00" }
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
                        onboarding_step.set(InvoiceStep::Recipient);
                    }
                }
                Button {
                    text: "Payment Details",
                    size: ElementSize::Small,
                    right_icon: rsx! {
                        Icon { icon: ChevronRight, height: 20, width: 20, fill: "var(--base-fill-5)" }
                    },
                    status: None,
                    on_click: move |_| {
                        onboarding_step.set(InvoiceStep::PaymentMethod);
                    }
                }
            }
        }
    )
}
