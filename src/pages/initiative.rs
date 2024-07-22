use std::{ops::Deref, vec};

use blake2::{digest::consts::U32, Blake2b, Blake2s256, Digest};
use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use futures_util::{StreamExt, TryFutureExt};
use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::{
    components::{
        atoms::{
            button::Variant, dropdown::ElementSize, icon_button::Variant as IconButtonVariant,
            Arrow, Button, Icon, IconButton, Step, StepCard,
        },
        molecules::{
            InitiativeActions, InitiativeConfirmation, InitiativeInfo, InitiativeSettings,
        },
    },
    hooks::{
        use_accounts::use_accounts,
        use_initiative::{
            use_initiative, ActionItem, InitiativeData, InitiativeInfoContent,
            InitiativeInitContent,
        },
        use_notification::use_notification,
        use_our_navigator::use_our_navigator,
        use_session::use_session,
        use_spaces_client::use_spaces_client,
        use_tooltip::{use_tooltip, TooltipItem},
    },
    pages::onboarding::convert_to_jsvalue,
    services::kreivo::community_referenda::referendum_count,
};

#[derive(Clone, Debug)]
pub enum InitiativeStep {
    Info,
    Actions,
    Settings,
    Confirmation,
    None,
}

#[derive(Serialize)]
struct AccountMembership {
    id: String,
    membership_id: u32,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = window, js_name = topupThenInitiativeAddMembers)]
    async fn topup_then_initiative_add_members(
        community_id: u16,
        initiative_id: u16,
        membership_accounts_add: JsValue,
        membership_accounts_remove: JsValue,
        room_id: JsValue,
        remark: JsValue,
    ) -> Result<JsValue, JsValue>;
}

#[component]
pub fn Initiative(id: u16) -> Element {
    let i18 = use_i18();
    let mut initiative = use_initiative();
    let mut accounts = use_accounts();
    let mut session = use_session();
    let mut nav = use_our_navigator();
    let mut tooltip = use_tooltip();
    let mut notification = use_notification();
    let spaces_client = use_spaces_client();

    let mut onboarding_step = use_signal::<InitiativeStep>(|| InitiativeStep::Info);
    let mut onboarding_steps = use_signal::<Vec<InitiativeStep>>(|| {
        vec![
            InitiativeStep::Info,
            InitiativeStep::Actions,
            InitiativeStep::Settings,
            InitiativeStep::Confirmation,
            InitiativeStep::None,
        ]
    });

    let mut handle_required_inputs = use_signal::<bool>(|| false);

    let get_account = move || {
        let Some(user_session) = session.get() else {
            return None;
        };

        accounts.get_one(user_session.account_id)
    };

    use_before_render(move || {
        initiative.default();
    });

    rsx! {
        div { class: "page--initiative",
            div { class: "progress progress--steps-cube",
                Step {
                    is_active: matches!(*onboarding_step.read(), InitiativeStep::Info),
                    is_completed: false,
                    has_cube: true,
                    name: Some(translate!(i18, "initiative.steps.info.label")),
                    on_click: move |_| {
                        onboarding_step.set(InitiativeStep::Info);
                    },
                }
                Step {
                    is_active: matches!(*onboarding_step.read(), InitiativeStep::Actions),
                    is_completed: false,
                    has_cube: true,
                    name: Some(translate!(i18, "initiative.steps.actions.label")),
                    on_click: move |_| {
                        if initiative.get_info().name.is_empty() {
                            onboarding_step.set(InitiativeStep::Info);
                            handle_required_inputs.set(true);
                            return;
                        } else {
                            handle_required_inputs.set(false);
                        }
                        onboarding_step.set(InitiativeStep::Actions);
                    },
                }
            }
            div { class: "initiative__form",
                div { class: "form__wrapper form__wrapper--initiative",
                    h2 { class: "form__title",
                        {translate!(i18, "initiative.title")}
                    }
                    div { class: "steps__wrapper",
                        StepCard {
                            name: {translate!(i18, "initiative.steps.info.label")},
                            checked: matches!(*onboarding_step.read(), InitiativeStep::Info ),
                            body: rsx!(
                                div { class: "step-card__info",
                                    span { class: "step-card__title",
                                        {translate!(i18, "initiative.steps.info.label")}
                                    }
                                    IconButton {
                                        variant: IconButtonVariant::Round,
                                        size: ElementSize::Big,
                                        class: "button--drop bg--transparent",
                                        body: rsx!(
                                            Icon {
                                                class: if matches!(*onboarding_step.read(), InitiativeStep::Info ) { "rotate-180" } else { "rotate-0" },
                                                icon: Arrow,
                                                height: 24,
                                                width: 24,
                                                stroke_width: 2,
                                                stroke: "var(--fill-400)"
                                            }
                                        ),
                                        on_click: move |_| {
                                            if matches!(*onboarding_step.read(), InitiativeStep::Info ) {
                                                onboarding_step.set(InitiativeStep::None);
                                            } else {
                                                onboarding_step.set(InitiativeStep::Info);
                                            }
                                        }
                                    }
                                }
                            ),
                            editable: rsx!(
                                div { class: "step-card__editable",
                                    InitiativeInfo {
                                        error: handle_required_inputs()
                                    }
                                }
                            ),
                            on_change: move |_| {
                                onboarding_step.set(InitiativeStep::Info);
                            },
                        }
                        StepCard {
                            name: {translate!(i18, "initiative.steps.actions.label")},
                            checked: matches!(*onboarding_step.read(), InitiativeStep::Actions ),
                            body: rsx!(
                                div { class: "step-card__info",
                                    span { class: "step-card__title",
                                        {translate!(i18, "initiative.steps.actions.label")}
                                    }
                                    IconButton {
                                        variant: IconButtonVariant::Round,
                                        size: ElementSize::Big,
                                        class: "button--drop  bg--transparent",
                                        body: rsx!(
                                            Icon {
                                                class: if matches!(*onboarding_step.read(), InitiativeStep::Actions ) { "rotate-180" } else { "rotate-0" },
                                                icon: Arrow,
                                                height: 24,
                                                width: 24,
                                                stroke_width: 2,
                                                stroke: "var(--fill-400)"
                                            }
                                        ),
                                        on_click: move |_| {
                                            if matches!(*onboarding_step.read(), InitiativeStep::Actions ) {
                                                onboarding_step.set(InitiativeStep::None);
                                            } else {
                                                onboarding_step.set(InitiativeStep::Actions);
                                            }
                                        }
                                    }
                                }
                            ),
                            editable: rsx!(
                                div { class: "step-card__editable",
                                    InitiativeActions {}
                                }
                            ),
                            on_change: move |_| {
                                onboarding_step.set(InitiativeStep::Actions);
                            },
                        }
                    }
                }
            }
            div { class: "form__cta form__cta--initiatives",
                Button {
                    class: "",
                    text: translate!(i18, "initiative.cta.cancel"),
                    size: ElementSize::Small,
                    variant: Variant::Secondary,
                    on_click: move |_| {
                        nav.go_back();
                    },
                    status: None,
                }
                Button {
                    class: "",
                    text: translate!(i18, "initiative.cta.continue"),
                    size: ElementSize::Small,
                    on_click: move |_| {
                        spawn(
                            async move {
                                tooltip.handle_tooltip(TooltipItem {
                                    title: translate!(i18, "initiative.tips.loading.title"),
                                    body: translate!(i18, "initiative.tips.loading.description"),
                                    show: true
                                });

                                let last_initiative = referendum_count().await.map_err(|_| {
                                    log::warn!("Failed to get last initiative");
                                    translate!(i18, "errors.form.initiative_creation")
                                })?;

                                let response_bot = spaces_client
                                    .get()
                                    .create_initiative(InitiativeData {
                                        init: InitiativeInitContent {
                                            sender: session
                                                .get()
                                                .expect("should get a signer for the session")
                                                .address,
                                            is_admin: false,
                                        },
                                        info: InitiativeInfoContent {
                                            name: initiative.get_info().name,
                                            description: initiative.get_info().description,
                                            tags: initiative.get_info().categories,
                                            actions: initiative.get_actions(),
                                        },
                                    })
                                    .await
                                    .map_err(|_| {
                                        log::warn!("Failed to create off-chain");
                                        translate!(i18, "errors.form.initiative_creation")
                                    })?;

                                let room_id = response_bot.get_id();

                                let mut hasher = Blake2b::<U32>::new();
                                hasher.update(room_id.clone());
                                let res = hasher.finalize();
                                let res = res.deref();

                                let room_to_blake = format!("0x{}", hex::encode(res));

                                let add_members_action = initiative.get_actions().into_iter().filter_map(|action| {
                                    match action {
                                        ActionItem::AddMembers(add_members_action) => {
                                            Some(add_members_action.members.clone()
                                            .into_iter()
                                            .filter_map(|member|
                                                if !member.account.is_empty() { Some(member.account) } else { None }
                                            )
                                            .collect::<Vec<String>>())
                                        },
                                        _ => None
                                    }
                                }).collect::<Vec<Vec<String>>>();

                                let add_members_action = add_members_action.into_iter().flat_map(|v| v.into_iter()).collect::<Vec<String>>();

                                let membership_accounts_add = convert_to_jsvalue(&add_members_action)
                                .map_err(|_| {
                                    log::warn!("Malformed membership accounts add");
                                    translate!(i18, "errors.form.initiative_creation")
                                })?;

                                let membership_accounts_remove = convert_to_jsvalue(&Vec::<String>::new())
                                    .map_err(|_| {
                                        log::warn!("Malformed membership accounts remove");
                                        translate!(i18, "errors.form.initiative_creation")
                                    })?;

                                let room_id =
                                    convert_to_jsvalue(&room_id.clone()).map_err(|_| {
                                        log::warn!("Malformed room id");
                                        translate!(i18, "errors.form.initiative_creation")
                                    })?;

                                let remark =
                                    convert_to_jsvalue(&room_to_blake).map_err(|_| {
                                        log::warn!("Malformed remark");
                                        translate!(i18, "errors.form.initiative_creation")
                                    })?;

                                let response = topup_then_initiative_add_members(
                                    id,
                                    last_initiative,
                                    membership_accounts_add,
                                    membership_accounts_remove,
                                    room_id,
                                    remark,
                                )
                                .await;

                                log::info!("response initiative: {:?}", response);
                                tooltip.hide();

                                let path = format!("/dao/{id}/initiatives");
                                nav.push(vec![], &path);

                                Ok::<(), String>(())
                            }
                            .unwrap_or_else(move |e: String| {
                                tooltip.hide();
                                notification.handle_error(&e);
                            })
                        );
                    },
                    status: None,
                }
            }
        }
    }
}
