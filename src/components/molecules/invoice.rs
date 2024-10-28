use dioxus::prelude::*;

use crate::hooks::use_invoice::{
    InvoiceDetails, InvoiceItems, InvoiceTerms, InvoiceTransfer, InvoiceX,
};
#[derive(PartialEq, Props, Clone)]
pub struct InvoiceProps {
    logo: String,
    from: InvoiceX,
    to: InvoiceX,
    items: InvoiceItems,
    details: InvoiceDetails,
    terms: InvoiceTerms,
    transfer: InvoiceTransfer,
}
pub fn InvoiceView(props: InvoiceProps) -> Element {
    rsx!(
        section { class: "invoice",
            div { class: "row",
                h2 { class: "invoice__title", "Invoice" }
                img { class: "invoice__logo",
                    src: props.logo
                }
            }
            div { class: "invoice__table",
                div { class: "row invoice__section",
                    div { class: "invoice__column",
                        h6 { class: "invoice__column__title", "From" }
                        p { class: "invoice__column__content", {props.from.name}}
                        p { class: "invoice__column__content", {props.from.email}}
                        p { class: "invoice__column__content", {props.from.address}}
                        p { class: "invoice__column__content", {props.from.country}}
                    }
                    div { class: "invoice__column",
                        h6 { class: "invoice__column__title", "To" }
                        p { class: "invoice__column__content", {props.to.name}}
                        p { class: "invoice__column__content", {props.to.email}}
                        p { class: "invoice__column__content", {props.to.address}}
                        p { class: "invoice__column__content", {props.to.country}}
                    }
                    div { class: "invoice__column",
                        h6 { class: "invoice__column__title", "Details" }
                        p { class: "invoice__column__content--key-value",
                            span { class: "invoice__column__content", "Invoice no." }
                            span { class: "invoice__column__content", {props.details.id} }
                        }
                    }
                }

                div { class: "invoice__section",
                    div { class: "row invoice__row",
                        h6 { class: "invoice__column__title", "Item" }
                        h6 { class: "invoice__column__title", "Quantity" }
                        h6 { class: "invoice__column__title", "Amount" }
                    }
                    hr { class: "divider--table" }
                    for item in props.items.items {
                        div { class: "row invoice__row",
                            p { class: "invoice__column__content", {item.description}}
                            p { class: "invoice__column__content", {item.quantity.to_string()}}
                            p { class: "invoice__column__content", {format!("${}", item.amount)}}
                        }
                        hr { class: "divider--item" }
                    }
                    div { class: "row invoice__row",
                        div { class: "" }
                        div { class: "invoice__column__tax__title",
                            {format!("Sales Tax ({}%)", props.details.tax.percent)}
                        }
                        div { class: "invoice__column__tax__value",
                            {format!("${}", props.details.tax.value)}
                        }
                    }
                    hr { class: "divider--item" }
                    div { class: "row invoice__row",
                        div { class: "" }
                        div { class: "invoice__column__total__title",
                            "Total"
                        }
                        div { class: "invoice__column__total__value",
                            {format!("${}", props.details.total)}
                        }
                    }
                }

                div { class: "invoice__section",
                    div { class: "row invoice__row",
                        h6 { class: "invoice__column__title", "Terms" }
                        h6 { class: "invoice__column__title invoice__column-2x", "Memo" }
                    }
                    hr { class: "divider--table" }
                    div { class: "row invoice__row",
                        div { class: "invoice__column__content--column",
                            p { class: "row",
                                span { class: "invoice__content", "Invoice date" }
                                span { class: "invoice__content--tertiary", {props.terms.created_at} }
                            }
                            p { class: "row",
                                span { class: "invoice__content", "Due date" }
                                span { class: "invoice__content--tertiary", {props.terms.payment_at} }
                            }
                            p { class: "row",
                                span { class: "invoice__content", "Pay via" }
                                span { class: "invoice__content--tertiary", {props.terms.payment_via} }
                            }
                        }
                        p { class: "invoice__column__content invoice__column-2x",
                            {props.terms.memo}
                        }
                    }
                }

                div { class: "invoice__section",
                    div { class: "row invoice__row",
                        h6 { class: "invoice__column__title", "Transfer details" }
                    }
                    hr { class: "divider--table" }
                    div { class: "row invoice__row--unique",
                        div { class: "invoice__column__content",
                            p { class: "row",
                                span { class: "invoice__content", "Bank name" }
                                span { class: "invoice__content--tertiary", {props.transfer.name} }
                            }
                            p { class: "row",
                                span { class: "invoice__content", "Account name" }
                                span { class: "invoice__content--tertiary", {props.transfer.account} }
                            }
                            p { class: "row",
                                span { class: "invoice__content", "Account type" }
                                span { class: "invoice__content--tertiary", {props.transfer.account_type} }
                            }
                            p { class: "row",
                                span { class: "invoice__content", "Account number" }
                                span { class: "invoice__content--tertiary", {props.transfer.account_number} }
                            }
                            p { class: "row",
                                span { class: "invoice__content", "Routing number" }
                                span { class: "invoice__content--tertiary", {props.transfer.routing_number} }
                            }
                        }
                    }
                }
            }
        }
    )
}
