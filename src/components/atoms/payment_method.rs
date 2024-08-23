use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct PaymentMethodProps {
    title: String,
    fee: String,
    icon: Element,
}

pub fn PaymentMethod(props: PaymentMethodProps) -> Element {
    rsx!(
        section { class: "checkbox__content--payment",
            span {
                class: "checkbox-card__title",
                "{props.title}"
            }
            div { class: "checkbox__content__left",
                span {
                    class: "checkbox-card__fee",
                    "{props.fee}"
                }
                div {
                    class: "checkbox-card__media",
                    {props.icon}
                }
            }
        }
    )
}
