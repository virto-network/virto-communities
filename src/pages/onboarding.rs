use std::ops::Deref;

use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use futures_util::TryFutureExt;
use gloo::utils::format::JsValueSerdeExt;
use serde::Serialize;

use crate::{
    components::{
        atoms::{
            button::Variant, dropdown::ElementSize, ArrowLeft, ArrowRight, Attach, Button, Icon,
            Input, TextareaInput, Title, VirtoLogo,
        },
        molecules::{OnboardingBasics, OnboardingInvite, OnboardingManagement},
    },
    hooks::{use_accounts::use_accounts, use_onboard::use_onboard, use_session::use_session},
};
use serde_json::{to_value, Error, Value as JsonValue};
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug)]
pub enum OnboardingStep {
    Basics,
    Management,
    Invite,
}

#[derive(Serialize)]
struct Identity {
    pub display: String,
    // TODO: enable this to integrate the actual required fields by blockchain
    // pub legal: Option<String>,
    // pub web: Option<String>,
    // pub matrix: Option<String>,
    // pub pgpFingerprint: Option<JsValue>,
    // pub image: Option<JsValue>,
    // pub twitter: Option<JsValue>,
    // pub github: Option<JsValue>,
    // pub discord: Option<JsValue>,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
enum DecisionMethod {
    Membership,
    Rank,
    NativeToken,
    CommunityAsset { id: String, min_vote: i32 },
}

#[derive(Serialize)]
struct CommunityData {
    signer: String,
    community_id: i32,
    decision_method: DecisionMethod,
    identity: Identity,
}

fn convert_to_jsvalue<T: Serialize>(value: &T) -> Result<JsValue, Error> {
    to_value(value)
        .map(|t: serde_json::Value| JsValue::from_serde(&t))
        .unwrap_or_else(|e| Ok(JsValue::from_str("Error creating JsValue")))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = topupThenCreateCommunity)]
    async fn topup_then_create_community(
        communityId: JsValue,
        name: String,
        decisionMethod: JsValue,
        identity: JsValue,
    );
}

#[component]
pub fn Onboarding() -> Element {
    let i18 = use_i18();
    let mut onboard = use_onboard();
    let mut accounts = use_accounts();
    let mut session = use_session();

    let mut onboarding_step = use_signal::<OnboardingStep>(|| OnboardingStep::Basics);
    let mut onboarding_steps = use_signal::<Vec<OnboardingStep>>(|| {
        vec![
            OnboardingStep::Basics,
            OnboardingStep::Management,
            OnboardingStep::Invite,
        ]
    });

    onboard.default();

    let get_account = move || {
        let Some(user_session) = session.get() else {
            return None;
        };

        accounts.get_one(user_session.account_id)
    };

    rsx! {
        button {
            onclick: move |_| {
                spawn(async move {
                    let identity = Identity {
                        display: "Example Community".to_string(),
                    };
                    let decision_method = DecisionMethod::Membership;

                    let Ok(identity_js) = convert_to_jsvalue(&identity) else {
                        return;
                    };

                    let Ok(decision_method_js) = convert_to_jsvalue(&decision_method) else {
                        return;
                    };

                    let Some(account) = get_account() else {
                        return;
                    };

                    let community_id_js = JsValue::from(123);

                    let response = topup_then_create_community(community_id_js, identity.display.clone(), decision_method_js, identity_js).await;
                    // TODO: notify an error with an unwrap_or_else
                });
            },
            "click me",
        }
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
                            OnboardingStep::Basics => rsx!{OnboardingBasics {}},
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
