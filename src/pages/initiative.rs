use std::vec;

use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

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
        use_accounts::use_accounts, use_initiative::use_initiative,
        use_our_navigator::use_our_navigator, use_session::use_session,
        use_spaces_client::use_spaces_client,
    },
};

#[derive(Clone, Debug)]
pub enum InitiativeStep {
    Info,
    Actions,
    Settings,
    Confirmation,
    None,
}

#[component]
pub fn Initiative() -> Element {
    let i18 = use_i18();
    let mut initiative = use_initiative();
    let mut accounts = use_accounts();
    let mut session = use_session();
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
                Step {
                    is_active: matches!(*onboarding_step.read(), InitiativeStep::Settings),
                    is_completed: false,
                    has_cube: true,
                    name: Some(translate!(i18, "initiative.steps.settings.label")),
                    on_click: move |_| {
                        if initiative.get_info().name.is_empty() {
                            onboarding_step.set(InitiativeStep::Info);
                            handle_required_inputs.set(true);
                            return;
                        } else {
                            handle_required_inputs.set(false);
                        }
                        onboarding_step.set(InitiativeStep::Settings);
                    },
                }
                Step {
                    is_active: matches!(*onboarding_step.read(), InitiativeStep::Confirmation),
                    is_completed: false,
                    has_cube: true,
                    name: Some(translate!(i18, "initiative.steps.confirmation.label")),
                    on_click: move |_| {
                        if initiative.get_info().name.is_empty() {
                            onboarding_step.set(InitiativeStep::Info);
                            handle_required_inputs.set(true);
                            return;
                        } else {
                            handle_required_inputs.set(false);
                        }
                        onboarding_step.set(InitiativeStep::Confirmation);
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
                        StepCard {
                            name: {translate!(i18, "initiative.steps.settings.label")},
                            checked: matches!(*onboarding_step.read(), InitiativeStep::Settings ),
                            body: rsx!(
                                div { class: "step-card__info",
                                    span { class: "step-card__title",
                                        {translate!(i18, "initiative.steps.settings.label")}
                                    }
                                    IconButton {
                                        variant: IconButtonVariant::Round,
                                        size: ElementSize::Big,
                                        class: "button--drop  bg--transparent",
                                        body: rsx!(
                                            Icon {
                                                class: if matches!(*onboarding_step.read(), InitiativeStep::Settings ) { "rotate-180" } else { "rotate-0" },
                                                icon: Arrow,
                                                height: 24,
                                                width: 24,
                                                stroke_width: 2,
                                                stroke: "var(--fill-400)"
                                            }
                                        ),
                                        on_click: move |_| {
                                            if matches!(*onboarding_step.read(), InitiativeStep::Settings ) {
                                                onboarding_step.set(InitiativeStep::None);
                                            } else {
                                                onboarding_step.set(InitiativeStep::Settings);
                                            }
                                        }
                                    }
                                }
                            ),
                            editable: rsx!(
                                div { class: "step-card__editable",
                                    InitiativeSettings {}
                                }
                            ),
                            on_change: move |_| {
                                onboarding_step.set(InitiativeStep::Settings);
                            },
                        }
                        StepCard {
                            name: {translate!(i18, "initiative.steps.confirmation.label")},
                            checked: matches!(*onboarding_step.read(), InitiativeStep::Confirmation ),
                            body: rsx!(
                                div { class: "step-card__info",
                                    span { class: "step-card__title",
                                        {translate!(i18, "initiative.steps.confirmation.label")}
                                    }
                                    IconButton {
                                        variant: IconButtonVariant::Round,
                                        size: ElementSize::Big,
                                        class: "button--drop  bg--transparent",
                                        body: rsx!(
                                            Icon {
                                                class: if matches!(*onboarding_step.read(), InitiativeStep::Confirmation ) { "rotate-180" } else { "rotate-0" },
                                                icon: Arrow,
                                                height: 24,
                                                width: 24,
                                                stroke_width: 2,
                                                stroke: "var(--fill-400)"
                                            }
                                        ),
                                        on_click: move |_| {
                                            if matches!(*onboarding_step.read(), InitiativeStep::Confirmation ) {
                                                onboarding_step.set(InitiativeStep::None);
                                            } else {
                                                onboarding_step.set(InitiativeStep::Confirmation);
                                            }
                                        }
                                    }
                                }
                            ),
                            editable: rsx!(
                                div { class: "step-card__editable",
                                    InitiativeConfirmation {}
                                }
                            ),
                            on_change: move |_| {
                                onboarding_step.set(InitiativeStep::Confirmation);
                            },
                        }
                    }
                }
            }
            div { class: "form__cta form__cta--initiatives",
                p { class: "wip",
                    {translate!(i18, "initiative.disclaimer")}
                }
            }
        }
    }
}
