use std::ops::Deref;

use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::{
        atoms::{
            button::Variant, dropdown::ElementSize, ArrowLeft, ArrowRight, Attach, Button, Icon,
            Input, TextareaInput, Title, VirtoLogo,
        },
        molecules::{OnboardingBasics, OnboardingInvite, OnboardingManagement},
    },
    hooks::use_onboard::use_onboard,
};

#[derive(Clone, Debug)]
pub enum OnboardingStep {
    Basics,
    Management,
    Invite,
}

#[component]
pub fn Onboarding() -> Element {
    let i18 = use_i18();
    let mut onboard = use_onboard();

    let mut onboarding_step = use_signal::<OnboardingStep>(|| OnboardingStep::Basics);
    let mut onboarding_steps = use_signal::<Vec<OnboardingStep>>(|| {
        vec![
            OnboardingStep::Basics,
            OnboardingStep::Management,
            OnboardingStep::Invite,
        ]
    });

    onboard.default();

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
                                onclick: move |_| onboarding_step.set(OnboardingStep::Management)
                            }
                            button {
                                class: "step",
                                class: if matches!(*onboarding_step.read(), OnboardingStep::Invite) { "step--active" },
                                onclick: move |_| onboarding_step.set(OnboardingStep::Invite)
                            }
                        }
                        match *onboarding_step.read() {
                            OnboardingStep::Basics => rsx!(OnboardingBasics {}),
                            OnboardingStep::Management => rsx!(OnboardingManagement {}),
                            OnboardingStep::Invite => rsx!(OnboardingInvite {}),
                        }

                    }
                    div { class: "form__cta",
                        if !matches!(*onboarding_step.read(), OnboardingStep::Basics) {
                            Button {
                                class: "",
                                text: "Back",
                                size: ElementSize::Big,
                                variant: Variant::Secondary,
                                on_click: move |_| {
                                    let step = onboarding_step();
                                    match step {
                                        OnboardingStep::Basics => onboarding_step.set(OnboardingStep::Basics),
                                        OnboardingStep::Management => onboarding_step.set(OnboardingStep::Basics),
                                        OnboardingStep::Invite => onboarding_step.set(OnboardingStep::Management),
                                    }
                                },
                                status: None,
                                left_icon: rsx!(
                                    Icon {
                                        icon: ArrowLeft,
                                        height: 32,
                                        width: 32,
                                        stroke_width: 1,
                                        fill: "var(--text-primary)"
                                    }
                                ),
                            }
                        }
                        Button {
                            class: "",
                            text: "Continue",
                            size: ElementSize::Big,
                            on_click: move |_| {
                                let step = onboarding_step();
                                match step {
                                    OnboardingStep::Basics => onboarding_step.set(OnboardingStep::Management),
                                    OnboardingStep::Management => onboarding_step.set(OnboardingStep::Invite),
                                    OnboardingStep::Invite => onboarding_step.set(OnboardingStep::Management),
                                };
                            },
                            status: None,
                            right_icon: rsx!(
                                Icon {
                                    icon: ArrowRight,
                                    height: 32,
                                    width: 32,
                                    stroke_width: 1,
                                    fill: "var(--white)"
                                }
                            ),
                        }
                    }
                }
                div { class: "onboarding__image",
                    img {
                        src: match *onboarding_step.read() {
                            OnboardingStep::Basics => "images/window.png",
                            OnboardingStep::Management => "images/phone.png",
                            OnboardingStep::Invite => "images/faces.png",
                        }
                    }
                }
            }
        }
    }
}
