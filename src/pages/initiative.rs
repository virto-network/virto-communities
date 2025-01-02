use crate::{
    components::{
        atoms::{
            button::Variant, dropdown::ElementSize, icon_button::Variant as IconButtonVariant,
            Arrow, Button, Icon, IconButton, Step, StepCard,
        },
        molecules::{InitiativeActions, InitiativeInfo},
    },
    hooks::{
        use_accounts::use_accounts,
        use_initiative::{
            use_initiative, InitiativeData, InitiativeInfoContent, InitiativeInitContent,
        },
        use_notification::use_notification,
        use_our_navigator::use_our_navigator,
        use_session::use_session,
        use_spaces_client::use_spaces_client,
        use_tooltip::{use_tooltip, TooltipItem},
    },
    middlewares::is_signer_ready::is_signer_ready,
    pages::onboarding::convert_to_jsvalue,
    services::{
        kreivo::{community_referenda::referendum_count, timestamp::now},
        kusama::system::number,
    },
};
use dioxus::{logger::tracing::{debug, warn, info}, prelude::*};
use dioxus_i18n::t;
use futures_util::TryFutureExt;
use wasm_bindgen::prelude::*;
#[derive(Clone, Debug)]
pub enum InitiativeStep {
    Info,
    Actions,
    Settings,
    Confirmation,
    None,
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = window, js_name = initiativeSetup)]
    pub async fn initiative_setup(
        community_id: u16,
        initiative_id: u16,
        room_id: JsValue,
        remark: JsValue,
        membership_accounts_add: JsValue,
        membership_accounts_remove: JsValue,
        periods_treasury_request: JsValue,
        proposals_voting_open_gov: JsValue,
        community_transfers: JsValue,
    ) -> Result<JsValue, JsValue>;
}

#[component]
pub fn Initiative(id: u16) -> Element {
    
    let mut initiative = use_initiative();
    let accounts = use_accounts();
    let session = use_session();
    let nav = use_our_navigator();
    let mut tooltip = use_tooltip();
    let mut notification = use_notification();
    let spaces_client = use_spaces_client();
    let mut onboarding_step = use_signal::<InitiativeStep>(|| InitiativeStep::Info);
    let mut handle_required_inputs = use_signal::<bool>(|| false);
    use_before_render(move || {
        initiative.default();
    });
    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        if is_signer_ready(accounts, notification)().is_err() {
            nav.push(vec![], &format!("/dao/{}/initiatives", id));
        };
    });
    rsx! {
        div { class: "page--initiative",
            div { class: "progress progress--steps-cube",
                Step {
                    is_active: matches!(*onboarding_step.read(), InitiativeStep::Info),
                    is_completed: false,
                    has_cube: true,
                    name: Some(t!("initiative-steps-info-label")),
                    on_click: move |_| {
                        onboarding_step.set(InitiativeStep::Info);
                    }
                }
                Step {
                    is_active: matches!(*onboarding_step.read(), InitiativeStep::Actions),
                    is_completed: false,
                    has_cube: true,
                    name: Some(t!("initiative-steps-actions-label")),
                    on_click: move |_| {
                        if initiative.get_info().name.is_empty() {
                            onboarding_step.set(InitiativeStep::Info);
                            handle_required_inputs.set(true);
                            return;
                        } else {
                            handle_required_inputs.set(false);
                        }
                        onboarding_step.set(InitiativeStep::Actions);
                    }
                }
            }
            div { class: "initiative__form",
                div { class: "form__wrapper form__wrapper--initiative",
                    h2 { class: "form__title", {t!("initiative-title")} }
                    div { class: "steps__wrapper",
                        StepCard {
                            name: t!("initiative-steps-info-label"),
                            checked: matches!(*onboarding_step.read(), InitiativeStep::Info),
                            body: rsx! {
                                div { class: "step-card__info",
                                    span { class: "step-card__title",
                                        {
                                        t!("initiative-steps-info-label") }
                                    }
                                    IconButton {
                                        variant: IconButtonVariant::Round,
                                        size: ElementSize::Big,
                                        class: "button--drop bg--transparent",
                                        body: rsx! {
                                            Icon {
                                                class: if matches!(*onboarding_step.read(), InitiativeStep::Info) {
                                                    "rotate-180"
                                                } else {
                                                    "rotate-0"
                                                },
                                                icon: Arrow,
                                                height: 24,
                                                width: 24,
                                                stroke_width: 2,
                                                stroke: "var(--fill-400)"
                                            }
                                        },
                                        on_click: move |_| {
                                            if matches!(*onboarding_step.read(), InitiativeStep::Info) {
                                                onboarding_step.set(InitiativeStep::None);
                                            } else {
                                                onboarding_step.set(InitiativeStep::Info);
                                            }
                                        }
                                    }
                                }
                            },
                            editable: rsx! {
                                div { class: "step-card__editable",
                                    InitiativeInfo { error: handle_required_inputs() }
                                }
                            },
                            on_change: move |_| {
                                onboarding_step.set(InitiativeStep::Info);
                            }
                        }
                        StepCard {
                            name: t!("initiative-steps-actions-label"),
                            checked: matches!(*onboarding_step.read(), InitiativeStep::Actions),
                            body: rsx! {
                                div { class: "step-card__info",
                                    span { class: "step-card__title",
                                        {
                                        t!("initiative-steps-actions-label") }
                                    }
                                    IconButton {
                                        variant: IconButtonVariant::Round,
                                        size: ElementSize::Big,
                                        class: "button--drop  bg--transparent",
                                        body: rsx! {
                                            Icon {
                                                class: if matches!(*onboarding_step.read(), InitiativeStep::Actions) {
                                                    "rotate-180"
                                                } else {
                                                    "rotate-0"
                                                },
                                                icon: Arrow,
                                                height: 24,
                                                width: 24,
                                                stroke_width: 2,
                                                stroke: "var(--fill-400)"
                                            }
                                        },
                                        on_click: move |_| {
                                            if matches!(*onboarding_step.read(), InitiativeStep::Actions) {
                                                onboarding_step.set(InitiativeStep::None);
                                            } else {
                                                onboarding_step.set(InitiativeStep::Actions);
                                            }
                                        }
                                    }
                                }
                            },
                            editable: rsx! {
                                div { class: "step-card__editable", InitiativeActions {} }
                            },
                            on_change: move |_| {
                                onboarding_step.set(InitiativeStep::Actions);
                            }
                        }
                    }
                }
            }
            div { class: "form__cta form__cta--initiatives",
                Button {
                    class: "",
                    text: t!("initiative-cta-cancel"),
                    size: ElementSize::Small,
                    variant: Variant::Secondary,
                    on_click: move |_| {
                        nav.go_back();
                    },
                    status: None
                }
                Button {
                    class: "",
                    text: t!("initiative-cta-continue"),
                    size: ElementSize::Small,
                    disabled: !initiative.check() || initiative.get_info().name.is_empty(),
                    on_click: move |_| {
                        spawn(
                            async move {
                                tooltip
                                    .handle_tooltip(TooltipItem {
                                        title: t!("initiative-tips-loading-title"),
                                        body: t!("initiative-tips-loading-description"),
                                        show: true,
                                    });
                                let last_initiative = referendum_count()
                                    .await
                                    .map_err(|_| {
                                        warn!("Failed to get last initiative");
                                        t!("errors-form-initiative_creation")
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
                                        warn!("Failed to create off-chain");
                                        t!("errors-form-initiative_creation")
                                    })?;
                                let room_id = response_bot.get_id();
                                let current_block = number()
                                    .await
                                    .map_err(|_| {
                                        warn!("Failed to get last block kusama");
                                        t!("errors-form-initiative_creation")
                                    })?;
                                let now_kusama = now()
                                    .await
                                    .map_err(|_| {
                                        warn!("Failed to get timestamp kusama");
                                        t!("errors-form-initiative_creation")
                                    })?;
                                debug!("{} {}", current_block, now_kusama);
                                let add_members_action = initiative.filter_valid_address_add_members();
                                debug!("add_members_action: {:?}", add_members_action);
                                let treasury_action = initiative
                                    .convert_treasury_to_period(current_block, now_kusama);
                                debug!("treasury {:?}", treasury_action);
                                let votiong_open_gov_action = initiative.filter_valid_voting_open_gov();
                                let votiong_open_gov_action = votiong_open_gov_action
                                    .into_iter()
                                    .map(|v| v.serialize_vote_type())
                                    .collect::<Vec<serde_json::Value>>();
                                debug!("votiong_open_gov_action {:?}", votiong_open_gov_action);
                                let community_transfer_action = initiative
                                    .filter_valid_community_transfer();
                                debug!("community_transfer_action {:?}", community_transfer_action);
                                let votiong_open_gov_action = convert_to_jsvalue(
                                        &votiong_open_gov_action,
                                    )
                                    .map_err(|_| {
                                        warn!("Malformed voting open gov");
                                        t!("errors-form-initiative_creation")
                                    })?;
                                let community_transfer_action = convert_to_jsvalue(
                                        &community_transfer_action,
                                    )
                                    .map_err(|_| {
                                        warn!("Malformed voting open gov");
                                        t!("errors-form-initiative_creation")
                                    })?;
                                let treasury_action = convert_to_jsvalue(&treasury_action)
                                    .map_err(|_| {
                                        warn!("Malformed membership accounts add");
                                        t!("errors-form-initiative_creation")
                                    })?;
                                let membership_accounts_add = convert_to_jsvalue(&add_members_action)
                                    .map_err(|_| {
                                        warn!("Malformed membership accounts add");
                                        t!("errors-form-initiative_creation")
                                    })?;
                                let membership_accounts_remove = convert_to_jsvalue(
                                        &Vec::<String>::new(),
                                    )
                                    .map_err(|_| {
                                        warn!("Malformed membership accounts remove");
                                        t!("errors-form-initiative_creation")
                                    })?;
                                let room_id = convert_to_jsvalue(&room_id.clone())
                                    .map_err(|_| {
                                        warn!("Malformed room id");
                                        t!("errors-form-initiative_creation")
                                    })?;
                                let remark = convert_to_jsvalue(&initiative.get_info().name)
                                    .map_err(|_| {
                                        warn!("Malformed remark");
                                        t!("errors-form-initiative_creation")
                                    })?;
                                initiative_setup(
                                        id,
                                        last_initiative,
                                        room_id,
                                        remark,
                                        membership_accounts_add,
                                        membership_accounts_remove,
                                        treasury_action,
                                        votiong_open_gov_action,
                                        community_transfer_action,
                                    )
                                    .await
                                    .map_err(|e| {
                                        warn!("Failed to create initiative {:?}", e);
                                        String::from("Failed to create initiative")
                                    })?;
                                tooltip.hide();
                                info!("created initiative {:?}", initiative.get_info().name);
                                let path = format!("/dao/{id}/initiatives");
                                nav.push(vec![], &path);
                                Ok::<(), String>(())
                            }
                                .unwrap_or_else(move |e: String| {
                                    tooltip.hide();
                                    notification.handle_error(&e);
                                }),
                        );
                    },
                    status: None
                }
            }
        }
    }
}
