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
    hooks::{
        use_accounts::use_accounts,
        use_attach::use_attach,
        use_notification::{
            use_notification, NotificationHandle, NotificationHandler, NotificationItem,
            NotificationVariant,
        },
        use_onboard::{use_onboard, BasicsForm, Invitations},
        use_session::use_session,
        use_tooltip::{use_tooltip, TooltipItem},
    },
    services::bot::create::{create, CommunitySpace},
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
#[serde(rename_all = "camelCase")]
struct TopupDetails {
    pub signer_in_kreivo: u32,
    pub community_account_in_kreivo: u32,
    pub community_account_in_people: u32,
}

#[derive(Serialize)]
struct Identity {
    pub display: String,
    // TODO: enable this to integrate the actual required fields by blockchain
    // pub legal: Option<String>,
    pub web: Option<String>,
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
        community_id: u16,
        name: String,
        decision_method: JsValue,
        maybe_identity: JsValue,
        maybe_memberships: JsValue,
        maybe_topup: JsValue,
    );
}

#[component]
pub fn Onboarding() -> Element {
    let i18 = use_i18();
    let nav = use_navigator();
    let mut onboard = use_onboard();
    let mut accounts = use_accounts();
    let mut session = use_session();
    let mut attach = use_attach();
    let mut tooltip = use_tooltip();
    let mut notification = use_notification();
    let mut to_pay = consume_context::<Signal<f64>>();

    let mut onboarding_step = use_signal::<OnboardingStep>(|| OnboardingStep::Basics);
    let mut onboarding_steps = use_signal::<Vec<OnboardingStep>>(|| {
        vec![
            OnboardingStep::Basics,
            OnboardingStep::Management,
            OnboardingStep::Invite,
        ]
    });

    let mut handle_required_inputs = use_signal::<bool>(|| false);

    use_before_render(move || {
        onboard.default();
    });

    use_drop(move || attach.reset());

    let get_account = move || {
        let Some(user_session) = session.get() else {
            return None;
        };

        accounts.get_one(user_session.account_id)
    };

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
                                    if onboard.get_basics().name.is_empty() || onboard.get_basics().industry.is_empty() {
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
                                    if onboard.get_basics().name.is_empty() || onboard.get_basics().industry.is_empty() {
                                        onboarding_step.set(OnboardingStep::Basics);
                                        handle_required_inputs.set(true);
                                        return;
                                    } else {
                                        handle_required_inputs.set(false);
                                    }
                                    onboarding_step.set(OnboardingStep::Invite);
                                }
                            }
                        }
                        match *onboarding_step.read() {
                            OnboardingStep::Basics => rsx!(OnboardingBasics {
                                error: handle_required_inputs()
                            }),
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

                        if !matches!(onboarding_step(), OnboardingStep::Invite) {
                            Button {
                                class: "",
                                text: "Continue",
                                size: ElementSize::Big,
                                on_click: move |_| {
                                    if onboard.get_basics().name.is_empty() || onboard.get_basics().industry.is_empty() {
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
                                        OnboardingStep::Invite => {

                                        },
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
                        } else {
                            Button {
                                class: "",
                                text: format!("Pay: {:.2} KSM", to_pay()),
                                size: ElementSize::Big,
                                on_click: move |_| {
                                    if onboard.get_basics().name.is_empty() || onboard.get_basics().industry.is_empty() {
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
                                        OnboardingStep::Invite => {
                                            spawn(async move {
                                                let community = CommunitySpace {
                                                    name: onboard.get_basics().name,
                                                    logo: onboard.get_basics().logo,
                                                    description: if onboard.get_basics().description.is_empty() {None} else {Some(onboard.get_basics().description)},
                                                    industry: onboard.get_basics().industry
                                                };

                                                tooltip.handle_tooltip(TooltipItem {
                                                    title: translate!(i18, "onboard.tips.loading.title"),
                                                    body: translate!(i18, "onboard.tips.loading.description"),
                                                    show: true
                                                });

                                                let Ok(decision_method) = convert_to_jsvalue(&DecisionMethod::Membership) else {
                                                    return;
                                                };

                                                let response = create(community).await.expect("Should return the room id");

                                                let identity = Identity {
                                                    display: onboard.get_basics().name,
                                                    web: Some(response.get_id())
                                                };

                                                let Ok(encoded_identity) = convert_to_jsvalue(&identity) else {
                                                    return;
                                                };

                                                let Some(account) = get_account() else {
                                                    return;
                                                };

                                                let members = onboard.get_invitations()
                                                    .into_iter()
                                                    .filter_map(|invitation| if !invitation.account.is_empty() { Some(invitation.account) } else { None })
                                                    .collect::<Vec<String>>();

                                                log::info!("{:?}", members);

                                                let Ok(membership_accounts) = convert_to_jsvalue(&members) else { return; };

                                                let response = topup_then_create_community(
                                                    1000,
                                                    identity.display.clone(),
                                                    decision_method,
                                                    encoded_identity,
                                                    membership_accounts,
                                                    JsValue::UNDEFINED
                                                ).await;

                                                log::info!("response create community onchain: {:?}", response);

                                                tooltip.hide();
                                                notification.handle_notification(
                                                    NotificationItem {
                                                        title: "Community has been created".to_string(),
                                                        body: "Community has been created".to_string(),
                                                        variant: NotificationVariant::Success,
                                                        show: true,
                                                        handle: NotificationHandle {value: NotificationHandler::None}
                                                    }
                                                );
                                                nav.push("/");
                                                // TODO: notify an error with an unwrap_or_else
                                            });
                                        },
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
