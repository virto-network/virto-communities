use std::str::FromStr;

use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use futures_util::TryFutureExt;
use wasm_bindgen::prelude::*;
use crate::{
    components::{
        atoms::{
            button::Variant, dropdown::ElementSize,
            icon_button::Variant as IconButtonVariant, Arrow, Button, Icon, IconButton,
            Step, StepCard,
        },
        molecules::{InitiativeActions, InitiativeInfo},
    }, hooks::{
        use_initiative::{
            use_initiative, ActionItem, InitiativeData, InitiativeInfoContent,
            InitiativeInitContent, KusamaTreasury, KusamaTreasuryPeriod, VotingOpenGov,
        },
        use_notification::use_notification, use_our_navigator::use_our_navigator,
        use_session::use_session, use_spaces_client::use_spaces_client,
        use_tooltip::{use_tooltip, TooltipItem},
    }, pages::payment::convert_to_jsvalue, services::{
        kreivo::{community_referenda::referendum_count, timestamp::now},
        kusama::system::number,
    }
};
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
    #[wasm_bindgen(catch, js_namespace = window, js_name = topupThenInitiativeSetup)]
    pub async fn topup_then_initiative_setup(
        community_id: u16,
        initiative_id: u16,
        room_id: JsValue,
        remark: JsValue,
        membership_accounts_add: JsValue,
        membership_accounts_remove: JsValue,
        periods_treasury_request: JsValue,
        proposals_voting_open_gov: JsValue,
    ) -> Result<JsValue, JsValue>;
}
const BLOCK_TIME_IN_SECONDS: i64 = 6;
#[component]
pub fn Initiative(id: u16) -> Element {
    let i18 = use_i18();
    let mut initiative = use_initiative();
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
                    }
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
                    }
                }
            }
            div { class: "initiative__form",
                div { class: "form__wrapper form__wrapper--initiative",
                    h2 { class: "form__title", {translate!(i18, "initiative.title")} }
                    div { class: "steps__wrapper",
                        StepCard {
                            name: translate!(i18, "initiative.steps.info.label"),
                            checked: matches!(*onboarding_step.read(), InitiativeStep::Info),
                            body: rsx!(
                                div { class : "step-card__info", span { class : "step-card__title", {
                                translate!(i18, "initiative.steps.info.label") } } IconButton { variant :
                                IconButtonVariant::Round, size : ElementSize::Big, class :
                                "button--drop bg--transparent", body : rsx!(Icon { class : if matches!(*
                                onboarding_step.read(), InitiativeStep::Info) { "rotate-180" } else { "rotate-0"
                                }, icon : Arrow, height : 24, width : 24, stroke_width : 2, stroke :
                                "var(--fill-400)" }), on_click : move | _ | { if matches!(* onboarding_step
                                .read(), InitiativeStep::Info) { onboarding_step.set(InitiativeStep::None); }
                                else { onboarding_step.set(InitiativeStep::Info); } } } }
                            ),
                            editable: rsx!(
                                div { class : "step-card__editable", InitiativeInfo { error :
                                handle_required_inputs() } }
                            ),
                            on_change: move |_| {
                                onboarding_step.set(InitiativeStep::Info);
                            }
                        }
                        StepCard {
                            name: translate!(i18, "initiative.steps.actions.label"),
                            checked: matches!(*onboarding_step.read(), InitiativeStep::Actions),
                            body: rsx!(
                                div { class : "step-card__info", span { class : "step-card__title", {
                                translate!(i18, "initiative.steps.actions.label") } } IconButton { variant :
                                IconButtonVariant::Round, size : ElementSize::Big, class :
                                "button--drop  bg--transparent", body : rsx!(Icon { class : if matches!(*
                                onboarding_step.read(), InitiativeStep::Actions) { "rotate-180" } else {
                                "rotate-0" }, icon : Arrow, height : 24, width : 24, stroke_width : 2, stroke :
                                "var(--fill-400)" }), on_click : move | _ | { if matches!(* onboarding_step
                                .read(), InitiativeStep::Actions) { onboarding_step.set(InitiativeStep::None); }
                                else { onboarding_step.set(InitiativeStep::Actions); } } } }
                            ),
                            editable: rsx!(div { class : "step-card__editable", InitiativeActions {} }),
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
                    text: translate!(i18, "initiative.cta.cancel"),
                    size: ElementSize::Small,
                    variant: Variant::Secondary,
                    on_click: move |_| {
                        nav.go_back();
                    },
                    status: None
                }
                Button {
                    class: "",
                    text: translate!(i18, "initiative.cta.continue"),
                    size: ElementSize::Small,
                    on_click: move |_| {
                        spawn(
                            async move {
                                tooltip
                                    .handle_tooltip(TooltipItem {
                                        title: translate!(i18, "initiative.tips.loading.title"),
                                        body: translate!(i18, "initiative.tips.loading.description"),
                                        show: true,
                                    });
                                let last_initiative = referendum_count()
                                    .await
                                    .map_err(|_| {
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
                                let add_members_action = initiative
                                    .get_actions()
                                    .into_iter()
                                    .filter_map(|action| {
                                        match action {
                                            ActionItem::AddMembers(add_members_action) => {
                                                Some(
                                                    add_members_action
                                                        .members
                                                        .clone()
                                                        .into_iter()
                                                        .filter_map(|member| {
                                                            if !member.account.is_empty() {
                                                                Some(member.account)
                                                            } else {
                                                                None
                                                            }
                                                        })
                                                        .collect::<Vec<String>>(),
                                                )
                                            }
                                            _ => None,
                                        }
                                    })
                                    .collect::<Vec<Vec<String>>>();
                                let add_members_action = add_members_action
                                    .into_iter()
                                    .flat_map(|v| v.into_iter())
                                    .collect::<Vec<String>>();
                                log::info!("add_members_action: {:?}", add_members_action);
                                let current_block = number()
                                    .await
                                    .map_err(|_| {
                                        log::warn!("Failed to get last block kusama");
                                        translate!(i18, "errors.form.initiative_creation")
                                    })?;
                                let now_kusama = now()
                                    .await
                                    .map_err(|_| {
                                        log::warn!("Failed to get timestamp kusama");
                                        translate!(i18, "errors.form.initiative_creation")
                                    })?;
                                log::info!("{} {}", current_block, now_kusama);
                                let treasury_action = initiative
                                    .get_actions()
                                    .into_iter()
                                    .filter_map(|action| {
                                        match action {
                                            ActionItem::KusamaTreasury(treasury_action) => {
                                                Some(
                                                    treasury_action
                                                        .periods
                                                        .clone()
                                                        .into_iter()
                                                        .filter_map(|period| {
                                                            if period.amount > 0 {
                                                                Some(
                                                                    convert_treasury_to_period(
                                                                        period,
                                                                        current_block,
                                                                        now_kusama,
                                                                    ),
                                                                )
                                                            } else {
                                                                None
                                                            }
                                                        })
                                                        .collect::<Vec<KusamaTreasuryPeriod>>(),
                                                )
                                            }
                                            _ => None,
                                        }
                                    })
                                    .collect::<Vec<Vec<KusamaTreasuryPeriod>>>();
                                let treasury_action = treasury_action
                                    .into_iter()
                                    .flat_map(|v| v.into_iter())
                                    .collect::<Vec<KusamaTreasuryPeriod>>();
                                log::info!("treasury {:?}", treasury_action);
                                let votiong_open_gov_action = initiative
                                    .get_actions()
                                    .into_iter()
                                    .filter_map(|action| {
                                        match action {
                                            ActionItem::VotingOpenGov(votiong_open_gov_action) => {
                                                Some(
                                                    votiong_open_gov_action
                                                        .proposals
                                                        .clone()
                                                        .into_iter()
                                                        .filter_map(|proposal| {
                                                            if proposal.poll_index > 0 { Some(proposal) } else { None }
                                                        })
                                                        .collect::<Vec<VotingOpenGov>>(),
                                                )
                                            }
                                            _ => None,
                                        }
                                    })
                                    .collect::<Vec<Vec<VotingOpenGov>>>();
                                let votiong_open_gov_action = votiong_open_gov_action
                                    .into_iter()
                                    .flat_map(|v| v.into_iter())
                                    .collect::<Vec<VotingOpenGov>>();
                                let votiong_open_gov_action = votiong_open_gov_action
                                    .into_iter()
                                    .map(|v| v.serialize_vote_type())
                                    .collect::<Vec<serde_json::Value>>();
                                log::info!("votiong_open_gov_action {:?}", votiong_open_gov_action);
                                let votiong_open_gov_action = convert_to_jsvalue(
                                        &votiong_open_gov_action,
                                    )
                                    .map_err(|_| {
                                        log::warn!("Malformed voting open gov");
                                        translate!(i18, "errors.form.initiative_creation")
                                    })?;
                                let treasury_action = convert_to_jsvalue(&treasury_action)
                                    .map_err(|_| {
                                        log::warn!("Malformed membership accounts add");
                                        translate!(i18, "errors.form.initiative_creation")
                                    })?;
                                let membership_accounts_add = convert_to_jsvalue(&add_members_action)
                                    .map_err(|_| {
                                        log::warn!("Malformed membership accounts add");
                                        translate!(i18, "errors.form.initiative_creation")
                                    })?;
                                let membership_accounts_remove = convert_to_jsvalue(
                                        &Vec::<String>::new(),
                                    )
                                    .map_err(|_| {
                                        log::warn!("Malformed membership accounts remove");
                                        translate!(i18, "errors.form.initiative_creation")
                                    })?;
                                let room_id = convert_to_jsvalue(&room_id.clone())
                                    .map_err(|_| {
                                        log::warn!("Malformed room id");
                                        translate!(i18, "errors.form.initiative_creation")
                                    })?;
                                let remark = convert_to_jsvalue(&initiative.get_info().name)
                                    .map_err(|_| {
                                        log::warn!("Malformed remark");
                                        translate!(i18, "errors.form.initiative_creation")
                                    })?;
                                topup_then_initiative_setup(
                                        id,
                                        last_initiative,
                                        room_id,
                                        remark,
                                        membership_accounts_add,
                                        membership_accounts_remove,
                                        treasury_action,
                                        votiong_open_gov_action,
                                    )
                                    .await
                                    .map_err(|e| {
                                        log::warn!("Failed to create initiative {:?}", e);
                                        String::from("Failed to create initiative")
                                    })?;
                                tooltip.hide();
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
fn calculate_future_block(
    current_block: u32,
    current_date_millis: u64,
    future_date_str: &str,
) -> u32 {
    let future_date_naive = NaiveDate::from_str(future_date_str).expect("Invalid future date");
    let future = future_date_naive
        .and_hms_opt(0, 0, 0)
        .expect("Invalid future date");
    let future_date = DateTime::<Utc>::from_naive_utc_and_offset(future, Utc);

    let x = DateTime::from_timestamp(
        (current_date_millis / 1000).try_into().unwrap(),
        ((current_date_millis % 1000) * 1_000_000) as u32,
    )
    .expect("");

    let x = NaiveDateTime::from_str(&x.date_naive().to_string()).expect("Invalid calculated date");
    let current_date = DateTime::from_naive_utc_and_offset(x, Utc);

    let elapsed_time_in_seconds = (future_date - current_date).num_seconds();
    let blocks_to_add = elapsed_time_in_seconds / BLOCK_TIME_IN_SECONDS;
    (current_block + blocks_to_add as u32).into()
}
fn convert_treasury_to_period(
    treasury: KusamaTreasury,
    current_block: u32,
    current_date_millis: u64,
) -> KusamaTreasuryPeriod {
    if treasury.date != "" {
        let future_block = calculate_future_block(
            current_block,
            current_date_millis,
            &treasury.date,
        );
        KusamaTreasuryPeriod {
            blocks: Some(future_block as u64),
            amount: treasury.amount,
        }
    } else {
        KusamaTreasuryPeriod {
            blocks: None,
            amount: treasury.amount,
        }
    }
}
