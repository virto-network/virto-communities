use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct PaymentMethodProps {
    title: String,
    description: String,
    icon: Element,
}

pub fn ManagementMethod(props: PaymentMethodProps) -> Element {
    rsx!(
        div {
            class: "checkbox-card__media",
            {props.icon}
        }
        div {
            span {
                class: "checkbox-card__title",
                "{props.title}"
            }
            p {
                class: "checkbox-card__description",
                "{props.description}"
            }
        }
    )
}
