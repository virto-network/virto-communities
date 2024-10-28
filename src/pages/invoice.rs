use crate::{
    components::{
        atoms::{dropdown::ElementSize, icon_button::Variant, Close, Icon, IconButton},
        molecules::{
            invoice::InvoiceView,
            treasury::invoice::{
                AmountForm, Completed, DetailsForm, PaymentMethodForm, RecipientForm, ReviewForm,
            },
        },
    },
    hooks::{
        use_accounts::use_accounts, use_communities::use_communities, use_invoice::use_invoice,
        use_notification::use_notification, use_our_navigator::use_our_navigator,
        use_tabs::use_tabs, use_timestamp::use_timestamp, use_tooltip::use_tooltip,
    },
};
use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

pub enum InvoiceStep {
    Recipient,
    PaymentMethod,
    Amount,
    Details,
    Review,
    Completed,
}

#[component]
pub fn Invoice(id: u16) -> Element {
    let i18 = use_i18();
    let mut tooltip = use_tooltip();
    let mut communities = use_communities();
    let mut tabs = use_tabs();
    let nav = use_our_navigator();
    let notification = use_notification();
    let accounts = use_accounts();
    let timestamp = use_timestamp();
    let invoice = use_invoice();

    let mut onboarding_step = use_context::<Signal<InvoiceStep>>();

    use_effect(use_reactive(
        (&communities.get_communities().len(),),
        move |(len,)| {
            if len > 0 {
                if let Err(_) = communities.set_community(id) {
                    let path = format!("/");
                    nav.push(vec![], &path);
                };
            }
        },
    ));

    rsx! {
        div { class: "bill page--ghost grid-main",
            IconButton {
                class: "send__button--close",
                size: ElementSize::Small,
                variant: Variant::Ghost,
                body: rsx!(
                    Icon { icon : Close, height : 20, width : 20, stroke_width : 2, fill :
                    "var(--state-brand-primary)" }
                ),
                on_click: move |_| {
                    nav.go_back();
                }
            }
            div { style: "display: flex; gap: 16px; justify-content: center;",
                div { class: "bill__form__wrapper",
                    if !matches!(&*onboarding_step.read(), InvoiceStep::Completed) {
                        div { class: "card-treasury__head",
                            div { class: "card-treasury__left",
                                h5 { class: "dashboard__head__subtitle", "Invoicing"}
                                h3 { class: "dashboard__head__title", "Create Invoice"}
                            }
                        }
                    }
                    match &*onboarding_step.read() {
                        InvoiceStep::Recipient => rsx!(RecipientForm {}),
                        InvoiceStep::Details => rsx!(DetailsForm {}),
                        InvoiceStep::PaymentMethod => rsx!(PaymentMethodForm {}),
                        InvoiceStep::Amount => rsx!(AmountForm {}),
                        InvoiceStep::Review => rsx!(ReviewForm {}),
                        InvoiceStep::Completed => rsx!(Completed {}),
                    }
                }
                if !matches!(&*onboarding_step.read(), InvoiceStep::Completed) {
                    div { class: "bill__invoice__wrapper",
                        InvoiceView {
                            logo: String::new(),
                            from: invoice.get_from(),
                            to: invoice.get_to(),
                            details: invoice.get_details(),
                            items: invoice.get_items(),
                            terms: invoice.get_terms(),
                            transfer: invoice.get_transfer(),
                        }
                    }
                }
            }
        }
    }
}
