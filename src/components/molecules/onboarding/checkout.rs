use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::atoms::Title,
    hooks::{
        use_market_client::use_market_client,
        use_onboard::use_onboard,
    },
    services::market::types::Tokens,
};

const COMMUNITY_VALUE: f64 = 0.51;
const IDENTITY_VALUE: f64 = 0.11;
const MEMBERSHIP_VALUE: f64 = 0.3;

#[component]
pub fn OnboardingCheckout() -> Element {
    let i18 = use_i18();
    let onboard = use_onboard();
    let market_client = use_market_client();

    let mut to_pay = consume_context::<Signal<f64>>();
    let mut ksm_market_price = use_signal::<f64>(|| 0.0);

    let members = onboard
        .get_invitations()
        .into_iter()
        .filter_map(|invitation| {
            if !invitation.account.is_empty() {
                Some(invitation.account)
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    let membership_value = members.len() as f64 * MEMBERSHIP_VALUE;
    let subtotal_ksm = COMMUNITY_VALUE + IDENTITY_VALUE + membership_value;

    to_pay.set(subtotal_ksm);

    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        let Ok(price) = market_client.get().get_price_by_token(Tokens::KSM).await else {
            log::error!("Price not found for {}", Tokens::KSM.name());
            return;
        };

        ksm_market_price.set(price);
    });

    rsx!(
        div { class: "form__title",
            span { class: "label",
                {translate!(i18, "onboard.checkout.label")}
            }
            Title {
                text: translate!(i18, "onboard.checkout.title")
            }
        }
        div { class: "summary",
            h4 { class: "summary__subtitle", {translate!(i18, "onboard.checkout.summary.subtitle")} }
            article { class: "summary__details",
                section { class: "summary__item",
                    span { class: "summary__item__label", {translate!(i18, "onboard.checkout.summary.account.label")} }
                    div { class: "summary__item__values",
                        span { class: "summary__item__value", {format!("{:.2} KSM", COMMUNITY_VALUE)} }
                        if ksm_market_price() > 0.0 {
                            small { class: "summary__item__value", {format!("${:.2} USD", ksm_market_price() * COMMUNITY_VALUE)} }
                        }
                    }
                }
                section { class: "summary__item",
                    span { class: "summary__item__label", {translate!(i18, "onboard.checkout.summary.verification.label")} }
                    div { class: "summary__item__values",
                        span { class: "summary__item__value", {format!("{:.2} KSM", IDENTITY_VALUE)} }
                        if ksm_market_price() > 0.0 {
                            small { class: "summary__item__value", {format!("${:.2} USD", ksm_market_price() * IDENTITY_VALUE)} }
                        }
                    }
                }
                section { class: "summary__item",
                    span { class: "summary__item__label", {translate!(i18, "onboard.checkout.summary.membership.label")} }
                    div { class: "summary__item__values",
                        span { class: "summary__item__value", {format!("x{} ({:.2} KSM)", members.len(), MEMBERSHIP_VALUE)} }
                        if ksm_market_price() > 0.0 {
                            small { class: "summary__item__value", {format!("${:.2} USD", ksm_market_price() * MEMBERSHIP_VALUE)} }
                        }
                    }
                    div { class: "summary__item__values",
                        span { class: "summary__item__value", {format!("{:.2} KSM", membership_value)} }
                        if ksm_market_price() > 0.0 {
                            small { class: "summary__item__value", {format!("${:.2} USD", ksm_market_price() * membership_value)} }
                        }
                    }
                }
            }

            hr { class: "divider" }

            section { class: "summary__item",
                span { class: "summary__item__label--subtitle", {translate!(i18, "onboard.checkout.summary.subtotal.label")} }
                div { class: "summary__item__values",
                    span { class: "summary__item__value", {format!("{:.2} KSM", subtotal_ksm)} }
                    if ksm_market_price() > 0.0 {
                        small { class: "summary__item__value",
                            {
                                format!("${:.2} USD",
                                    round_price(ksm_market_price() * COMMUNITY_VALUE)
                                    + round_price(ksm_market_price() * IDENTITY_VALUE)
                                    + round_price(ksm_market_price() * membership_value)
                                )
                            }
                        }
                    }
                }
            }

            p { class: "summary__disclaimer", {translate!(i18, "onboard.checkout.summary.disclaimer")} }
        }
    )
}

fn round_price(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}
