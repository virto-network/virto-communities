use crate::{
    components::{
        atoms::{
            button::Variant, dropdown::ElementSize, ArrowLeft, ArrowRight, Button, Icon, VirtoLogo,
        },
        molecules::{OnboardingBasics, OnboardingCheckout, OnboardingInvite, OnboardingManagement},
    },
    hooks::{
        use_accounts::use_accounts, use_attach::use_attach, use_notification::use_notification,
        use_onboard::use_onboard, use_our_navigator::use_our_navigator,
        use_timestamp::use_timestamp,
    },
    middlewares::is_chain_available::is_chain_available,
};
use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
#[derive(Clone, Debug)]
pub enum OnboardingStep {
    Basics,
    Management,
    Invite,
    Checkout,
}

#[component]
pub fn Onboarding() -> Element {
    let i18 = use_i18();
    let accounts = use_accounts();
    let mut onboard = use_onboard();
    let mut attach = use_attach();
    let notification = use_notification();
    let nav = use_our_navigator();
    let timestamp = use_timestamp();

    let to_pay = consume_context::<Signal<f64>>();

    let mut onboarding_step = use_signal::<OnboardingStep>(|| OnboardingStep::Basics);

    let mut handle_required_inputs = use_signal::<bool>(|| false);
    use_before_render(move || {
        onboard.default();
    });
    use_drop(move || attach.reset());
    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        if accounts.is_active_account_an_admin() {
            nav.push(vec![], "/");
        };

        if let Err(_) = is_chain_available(i18, timestamp, notification)() {
            nav.push(vec![], "/");
        };
    });
    rsx! {
        div { class: "page page--onboarding",
            div { class: "row",
                div { class: "onboarding__form",
                    div { class: "form__wrapper",
                        Icon {
                            icon: VirtoLogo,
                            height: 64,
                            width: 64,
                            stroke_width: 1,
                            fill: "var(--color-lavanda-400)"
                        }
                        div { class: "progress progress--steps",
                            button {
                                class: "step",
                                class: if matches!(*onboarding_step.read(), OnboardingStep::Basics) { "step--active" },
                                onclick: move |_| {
                                    onboarding_step.set(OnboardingStep::Basics);
                                }
                            }
                            button {
                                class: "step",
                                class: if matches!(*onboarding_step.read(), OnboardingStep::Management) { "step--active" },
                                onclick: move |_| {
                                    if onboard.get_basics().name.is_empty()
                                        || onboard.get_basics().industry.is_empty()
                                    {
                                        onboarding_step.set(OnboardingStep::Basics);
                                        handle_required_inputs.set(true);
                                        return;
                                    } else {
                                        handle_required_inputs.set(false);
                                    }
                                    onboarding_step.set(OnboardingStep::Management);
                                }
                            }
                            button {
                                class: "step",
                                class: if matches!(*onboarding_step.read(), OnboardingStep::Invite) { "step--active" },
                                onclick: move |_| {
                                    if onboard.get_basics().name.is_empty()
                                        || onboard.get_basics().industry.is_empty()
                                    {
                                        onboarding_step.set(OnboardingStep::Basics);
                                        handle_required_inputs.set(true);
                                        return;
                                    } else {
                                        handle_required_inputs.set(false);
                                    }
                                    onboarding_step.set(OnboardingStep::Invite);
                                }
                            }
                            button {
                                class: "step",
                                class: if matches!(*onboarding_step.read(), OnboardingStep::Checkout) { "step--active" },
                                onclick: move |_| {
                                    if onboard.get_basics().name.is_empty() || onboard.get_basics().industry.is_empty() {
                                        onboarding_step.set(OnboardingStep::Basics);
                                        handle_required_inputs.set(true);
                                        return;
                                    } else {
                                        handle_required_inputs.set(false);
                                    }
                                    onboarding_step.set(OnboardingStep::Checkout);
                                }
                            }
                        }
                        match *onboarding_step.read() {
                            OnboardingStep::Basics => rsx!(OnboardingBasics {
                                error: handle_required_inputs()
                            }),
                            OnboardingStep::Management => rsx!(OnboardingManagement {}),
                            OnboardingStep::Invite => rsx!(OnboardingInvite {}),
                            OnboardingStep::Checkout => rsx!(OnboardingCheckout {}),
                        }
                    }
                    div { class: "form__cta",
                        if !matches!(*onboarding_step.read(), OnboardingStep::Basics) {
                            Button {
                                class: "",
                                text: translate!(i18, "onboard.management.cta.back"),
                                size: ElementSize::Big,
                                variant: Variant::Secondary,
                                on_click: move |_| {
                                    let step = onboarding_step();
                                    match step {
                                        OnboardingStep::Basics => onboarding_step.set(OnboardingStep::Basics),
                                        OnboardingStep::Management => onboarding_step.set(OnboardingStep::Basics),
                                        OnboardingStep::Invite => onboarding_step.set(OnboardingStep::Management),
                                        OnboardingStep::Checkout => onboarding_step.set(OnboardingStep::Invite),
                                    }
                                },
                                status: None,
                                left_icon: rsx!(
                                    Icon { icon : ArrowLeft, height : 32, width : 32, stroke_width : 1, fill :
                                    "var(--text-primary)" }
                                )
                            }
                        }
                        if !matches!(onboarding_step(), OnboardingStep::Checkout) {
                            Button {
                                class: "",
                                text: translate!(i18, "onboard.management.cta.next"),
                                size: ElementSize::Big,
                                on_click: move |_| {
                                    if onboard.get_basics().name.is_empty()
                                        || onboard.get_basics().industry.is_empty()
                                    {
                                        onboarding_step.set(OnboardingStep::Basics);
                                        handle_required_inputs.set(true);
                                        return;
                                    } else {
                                        handle_required_inputs.set(false);
                                    }
                                    let step = onboarding_step();
                                    match step {
                                        OnboardingStep::Basics => onboarding_step.set(OnboardingStep::Management),
                                        OnboardingStep::Management => onboarding_step.set(OnboardingStep::Invite),
                                        OnboardingStep::Invite => onboarding_step.set(OnboardingStep::Checkout),
                                        OnboardingStep::Checkout => {}
                                    };
                                },
                                status: None,
                                right_icon: rsx!(
                                    Icon { icon : ArrowRight, height : 32, width : 32, stroke_width : 1, fill :
                                    "var(--white)" }
                                )
                            }
                        } else {
                            Button {
                                class: "",
                                text: format!("{}: {:.2} KSM", translate!(i18, "onboard.checkout.cta.next"), to_pay()),
                                size: ElementSize::Big,
                                on_click: move |_| {
                                    if onboard.get_basics().name.is_empty()
                                        || onboard.get_basics().industry.is_empty()
                                    {
                                        onboarding_step.set(OnboardingStep::Basics);
                                        handle_required_inputs.set(true);
                                        return;
                                    } else {
                                        handle_required_inputs.set(false);
                                    }
                                    let step = onboarding_step();
                                    match step {
                                        OnboardingStep::Basics => {
                                            onboarding_step.set(OnboardingStep::Management);
                                        }
                                        OnboardingStep::Management => {
                                            onboarding_step.set(OnboardingStep::Invite);
                                        }
                                        OnboardingStep::Invite => {
                                            onboarding_step.set(OnboardingStep::Checkout);
                                        },
                                        OnboardingStep::Checkout => {
                                            nav.push(vec![], "/payment");
                                        }
                                    };
                                },
                                status: None,
                                right_icon: rsx!(
                                    Icon { icon : ArrowRight, height : 32, width : 32, stroke_width : 1, fill :
                                    "var(--white)" }
                                )
                            }
                        }
                    }
                }
                div { class: "onboarding__image",
                    img { src: match *onboarding_step.read() {
                            OnboardingStep::Basics => "images/window.png",
                            OnboardingStep::Management => "images/phone.png",
                            OnboardingStep::Invite | OnboardingStep::Checkout => "images/faces.png",
                        } }
                }
            }
        }
    }
}
