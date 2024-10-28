use dioxus::prelude::*;

use crate::components::atoms::{
    dropdown::ElementSize, icon_button::Variant as IconButtonVariant, input_tags::InputTagsEvent,
    Button, Checkbox, CheckboxCircleFill, Download, Icon, IconButton, InputTags, TelegramLogo,
};

use super::recipient::ControlProps;

#[component]
pub fn Completed(props: ControlProps) -> Element {
    rsx!(
        section { class: "send__form",
            Icon { icon: CheckboxCircleFill, height: 32, width: 32, fill: "var(--base-fill-5)" }
            h3 {class: "send__form__confirm", "You’ve sent a Bank Transfer payment"}
            p { class: "send__form__advice", "The payment should arrive within moments"}
            section { class: "send__form__transfer__wrapper",
                div { class: "send__form__transfer__details",
                    div { class: "send__form__transfer",
                        p { class: "send__form__details",
                            {format!("{} to {}", "Bank Transfer", "David Barinas")}
                        }
                        p { class: "send__form__amount",
                            {format!("${} USDT", "60")}
                        }
                    }
                    IconButton {
                        variant: IconButtonVariant::Round,
                        size: ElementSize::Small,
                        class: "button--avatar bg--state-primary-active",
                        body: rsx!(Icon { icon : Download, height : 20, width : 20, fill : "var(--fill-00)" }),
                        on_click: move |_| {

                        }
                    }
                }
                hr {class:"divider divider--table"}
                div { class: "send__form__transfer__dispatch",
                    InputTags {
                        message: "",
                        size: ElementSize::Small,
                        placeholder: "",
                        error: None,
                        maxlength: 5,
                        required: true,
                        on_input: move |_: InputTagsEvent| {

                        },
                        on_keypress: move |_| {},
                        on_click: move |_| {}
                    }
                    Checkbox {
                        id: "accept-credit-cards",
                        name: "accept-credit-cards",
                        label: "Include file.png in the receipt",
                        checked: false,
                        on_change: move |_| {}
                    }
                    Checkbox {
                        id: "accept-credit-cards",
                        name: "accept-credit-cards",
                        label: "Include 2ndfile.png in the receipt",
                        checked: false,
                        on_change: move |_| {}
                    }
                    div { class: "send__form__transfer__dispatch__cta",
                        p { class: "send__form__details", "Send 3 Emails"}
                        Button {
                            text: "Send All",
                            size: ElementSize::Small,
                            status: None,
                            left_icon: rsx! {
                                Icon { icon: TelegramLogo, height: 20, width: 20, fill: "var(--base-surface-1)" }
                            },
                            on_click: move |_| {
                                props.on_back.call(())
                            }
                        }
                    }
                }
            }
            div { class: "row",
                Button {
                    text: "Make Another Payment",
                    size: ElementSize::Small,
                    status: None,
                    on_click: move |_| {
                        props.on_back.call(())
                    }
                }
                Button {
                    text: "Go to Tunja Community",
                    size: ElementSize::Small,
                    status: None,
                    on_click: move |_| {
                        props.on_next.call(())
                    }
                }
            }
        }
    )
}
