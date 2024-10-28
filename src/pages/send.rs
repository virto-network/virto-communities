use crate::{
    components::{
        atoms::{dropdown::ElementSize, icon_button::Variant, Close, Icon, IconButton, Step},
        molecules::send::{
            AmountForm, Completed, InfoForm, PaymentMethodForm, Recipient, ReviewForm,
        },
    },
    hooks::{use_communities::use_communities, use_our_navigator::use_our_navigator},
};
use dioxus::prelude::*;

pub enum SendStep {
    Recipient,
    PaymentMethod,
    Amount,
    Info,
    Review,
    Completed,
}

#[component]
pub fn Send(id: u16) -> Element {
    let mut communities = use_communities();
    let nav = use_our_navigator();

    let mut onboarding_step = use_signal::<SendStep>(|| SendStep::Recipient);

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
        div { class: "send page--ghost grid-main",
            if !matches!(&*onboarding_step.read(), SendStep::Completed) {
                div { class: "card-treasury__head",
                    div { class: "card-treasury__left",
                        h5 { class: "dashboard__head__subtitle", {communities.get_community().name}}
                        h3 { class: "dashboard__head__title", "Send"}
                    }
                }
            } else {
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
            }
            div { style: "display: flex; gap: 16px;",
                if !matches!(&*onboarding_step.read(), SendStep::Completed) {
                    div { class: "progress__wrapper",
                        div { class: "progress progress--steps--list",
                            Step {
                                is_active: matches!(*onboarding_step.read(), SendStep::Recipient),
                                is_completed: false,
                                is_column: true,
                                name: Some("Recipient".to_string()),
                                on_click: move |_| {
                                    onboarding_step.set(SendStep::Recipient);
                                }
                            }
                            Step {
                                is_active: matches!(*onboarding_step.read(), SendStep::PaymentMethod),
                                is_completed: false,
                                is_column: true,
                                name: Some("Payment Method".to_string()),
                                on_click: move |_| {
                                    onboarding_step.set(SendStep::PaymentMethod);
                                }
                            }
                            Step {
                                is_active: matches!(*onboarding_step.read(), SendStep::Amount),
                                is_completed: false,
                                is_column: true,
                                name: Some("Amount & Source".to_string()),
                                on_click: move |_| {
                                    onboarding_step.set(SendStep::Amount);
                                }
                            }
                            Step {
                                is_active: matches!(*onboarding_step.read(), SendStep::Info),
                                is_completed: false,
                                is_column: true,
                                name: Some("Info".to_string()),
                                on_click: move |_| {
                                    onboarding_step.set(SendStep::Info);
                                }
                            }
                            Step {
                                is_active: matches!(*onboarding_step.read(), SendStep::Review),
                                is_completed: false,
                                is_column: true,
                                name: Some("Review".to_string()),
                                on_click: move |_| {
                                    onboarding_step.set(SendStep::Review);
                                }
                            }
                        }
                    }
                }
                div { class: "send__form__wrapper",
                    match &*onboarding_step.read() {
                        SendStep::Recipient => rsx!(Recipient {
                            on_back: move |_| {},
                            on_next: move |_| {onboarding_step.set(SendStep::PaymentMethod)}
                        }),
                        SendStep::PaymentMethod => rsx!(PaymentMethodForm {
                            on_back: move |_| {onboarding_step.set(SendStep::Recipient)},
                            on_next: move |_| {onboarding_step.set(SendStep::Amount)}
                        }),
                        SendStep::Amount => rsx!(AmountForm {
                            on_back: move |_| {onboarding_step.set(SendStep::PaymentMethod)},
                            on_next: move |_| {onboarding_step.set(SendStep::Info)}
                        }),
                        SendStep::Info => rsx!(InfoForm {
                            on_back: move |_| {onboarding_step.set(SendStep::Amount)},
                            on_next: move |_| {onboarding_step.set(SendStep::Review)}
                        }),
                        SendStep::Review => rsx!(ReviewForm {
                            on_back: move |_| {onboarding_step.set(SendStep::Info)},
                            on_next: move |_| {onboarding_step.set(SendStep::Completed)}
                        }),
                        SendStep::Completed => rsx!(Completed {
                            on_back: move |_| {onboarding_step.set(SendStep::Recipient)},
                            on_next: move |_| {nav.push(vec![], &format!("/dao/{}/plugins", id));}
                        })
                    }
                }
            }
        }
    }
}
