use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use futures_util::TryFutureExt;
use gloo::utils::format::JsValueSerdeExt;
use serde::Serialize;
use crate::{
    components::{
        atoms::{
            button::Variant, dropdown::ElementSize, ArrowLeft, ArrowRight, Button, Icon,
            VirtoLogo,
        },
        molecules::{OnboardingBasics, OnboardingInvite, OnboardingManagement},
    },
    hooks::{
        use_attach::use_attach,
        use_notification::{
            use_notification, NotificationHandle, NotificationHandler, NotificationItem,
            NotificationVariant,
        },
        use_onboard::use_onboard,
        use_our_navigator::use_our_navigator,
        use_spaces_client::use_spaces_client,
        use_tooltip::{use_tooltip, TooltipItem},
    },
    services::{bot::types::CommunitySpace, kreivo::community_track::tracksIds},
};
use serde_json::{to_value, Error};
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
    pub matrix: Option<String>,
}
#[derive(Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
enum DecisionMethod {
    Membership,
    _Rank,
    _NativeToken,
    _CommunityAsset { id: String, min_vote: i32 },
}
pub fn convert_to_jsvalue<T: Serialize>(value: &T) -> Result<JsValue, Error> {
    to_value(value)
        .map(|t: serde_json::Value| JsValue::from_serde(&t))
        .unwrap_or_else(|_| Ok(JsValue::from_str("Error creating JsValue")))
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = window, js_name = topupThenCreateCommunity)]
    async fn topup_then_create_community(
        community_id: u16,
        name: String,
        decision_method: JsValue,
        maybe_identity: JsValue,
        maybe_memberships: JsValue,
        maybe_topup: JsValue,
    ) -> Result<JsValue, JsValue>;
}
#[component]
pub fn Onboarding() -> Element {
    let i18 = use_i18();
    let mut onboard = use_onboard();
    let mut attach = use_attach();
    let mut tooltip = use_tooltip();
    let mut notification = use_notification();
    let spaces_client = use_spaces_client();
    let nav = use_our_navigator();

    let to_pay = consume_context::<Signal<f64>>();

    let mut id_number = use_signal::<String>(|| String::new());
    let mut onboarding_step = use_signal::<OnboardingStep>(|| OnboardingStep::Basics);

    let mut handle_required_inputs = use_signal::<bool>(|| false);
    use_before_render(move || {
        onboard.default();
    });
    use_drop(move || attach.reset());
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
                                text: translate!(i18, "onboard.management.cta.back"),
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
                                    Icon { icon : ArrowLeft, height : 32, width : 32, stroke_width : 1, fill :
                                    "var(--text-primary)" }
                                )
                            }
                        }
                        if !matches!(onboarding_step(), OnboardingStep::Invite) {
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
                                        OnboardingStep::Invite => {}
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
                                text: format!("{}: {:.2} KSM", translate!(i18, "onboard.invite.cta.next"), to_pay()),
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
                                            spawn({
                                                async move {
                                                    let community = CommunitySpace {
                                                        name: onboard.get_basics().name,
                                                        logo: onboard.get_basics().logo,
                                                        description: if onboard.get_basics().description.is_empty() {
                                                            None
                                                        } else {
                                                            Some(onboard.get_basics().description)
                                                        },
                                                        industry: onboard.get_basics().industry,
                                                    };
                                                    let response_track_ids = tracksIds()
                                                        .await
                                                        .map_err(|_| {
                                                            translate!(i18, "errors.form.community_creation")
                                                        })?;
                                                    let name_bytes = Vec::from(onboard.get_basics().name);
                                                    let community_id = name_bytes
                                                        .into_iter()
                                                        .fold(0u16, |acc, elem| acc + elem as u16);
                                                    let mut offset = 0u16;
                                                    while response_track_ids
                                                        .communities
                                                        .contains(&(community_id + offset))
                                                    {
                                                        offset += 1u16;
                                                    }
                                                    let current_id = community_id + offset;
                                                    id_number.set(current_id.to_string());
                                                    tooltip
                                                        .handle_tooltip(TooltipItem {
                                                            title: translate!(i18, "onboard.tips.loading.title"),
                                                            body: translate!(i18, "onboard.tips.loading.description"),
                                                            show: true,
                                                        });
                                                    let decision_method = convert_to_jsvalue(
                                                            &DecisionMethod::Membership,
                                                        )
                                                        .map_err(|_| {
                                                            log::warn!("Malformed decision method");
                                                            translate!(i18, "errors.form.community_creation")
                                                        })?;
                                                    let response = spaces_client
                                                        .get()
                                                        .create(community)
                                                        .await
                                                        .map_err(|_| {
                                                            translate!(i18, "errors.form.community_creation")
                                                        })?;
                                                    let identity = Identity {
                                                        display: onboard.get_basics().name,
                                                        matrix: Some(response.get_id()),
                                                    };
                                                    let encoded_identity = convert_to_jsvalue(&identity)
                                                        .map_err(|_| {
                                                            log::warn!("Malformed identity");
                                                            translate!(i18, "errors.form.community_creation")
                                                        })?;
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
                                                    let membership_accounts = convert_to_jsvalue(&members)
                                                        .map_err(|_| {
                                                            log::warn!("Malformed membership accounts");
                                                            translate!(i18, "errors.form.community_creation")
                                                        })?;
                                                    topup_then_create_community(
                                                            current_id,
                                                            identity.display.clone(),
                                                            decision_method,
                                                            encoded_identity,
                                                            membership_accounts,
                                                            JsValue::UNDEFINED,
                                                        )
                                                        .await
                                                        .map_err(|_| {
                                                            log::warn!("Error on xcm program");
                                                            translate!(i18, "errors.form.community_creation")
                                                        })?;
                                                    tooltip.hide();
                                                    notification
                                                        .handle_notification(NotificationItem {
                                                            title: translate!(i18, "onboard.tips.created.title"),
                                                            body: translate!(i18, "onboard.tips.created.description"),
                                                            variant: NotificationVariant::Success,
                                                            show: true,
                                                            handle: NotificationHandle {
                                                                value: NotificationHandler::None,
                                                            },
                                                        });
                                                    nav.push(Vec::new(), "/");
                                                    Ok::<(), String>(())
                                                }
                                                    .unwrap_or_else(move |e: String| {
                                                        tooltip.hide();
                                                        notification.handle_error(&e);
                                                    })
                                            });
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
                            OnboardingStep::Invite => "images/faces.png",
                        } }
                }
            }
        }
    }
}
