use crate::{
    components::{
        atoms::{dropdown::ElementSize, icon_button::Variant, Close, Icon, IconButton},
        molecules::{
            bill::{AmountForm, Completed, InfoForm, PaymentMethodForm, RecipientForm, ReviewForm},
            invoice::InvoiceView,
        },
    },
    hooks::{
        use_communities::use_communities, use_invoice::use_invoice,
        use_our_navigator::use_our_navigator,
    },
};
use dioxus::prelude::*;

pub enum BillStep {
    Recipient,
    PaymentMethod,
    Amount,
    Info,
    Review,
    Completed,
}

#[component]
pub fn Bill(id: u16) -> Element {
    let mut communities = use_communities();
    let nav = use_our_navigator();
    let invoice = use_invoice();

    let onboarding_step = use_context::<Signal<BillStep>>();

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
            div { style: "display: flex; gap: 16px;",
                div { class: "bill__form__wrapper",
                    if !matches!(&*onboarding_step.read(), BillStep::Completed) {
                        div { class: "card-treasury__head",
                            div { class: "card-treasury__left",
                                h5 { class: "dashboard__head__subtitle", {communities.get_community().name}}
                                h3 { class: "dashboard__head__title", "INV-902"}
                            }
                        }
                    }
                    match &*onboarding_step.read() {
                        BillStep::Recipient => rsx!(RecipientForm {}),
                        BillStep::PaymentMethod => rsx!(PaymentMethodForm {}),
                        BillStep::Amount => rsx!(AmountForm {}),
                        BillStep::Info => rsx!(InfoForm {}),
                        BillStep::Review => rsx!(ReviewForm {}),
                        BillStep::Completed => rsx!(Completed {}),
                    }
                }
                if !matches!(&*onboarding_step.read(), BillStep::Completed) {
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
