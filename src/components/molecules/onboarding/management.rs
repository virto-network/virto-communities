use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::atoms::{
        dropdown::ElementSize, ArrowLeft, Attach, CheckboxCard, Icon, Input, KeyFill, MedalFill, ShieldKeyholeFill, TeamFill, TextareaInput, Title
    },
    hooks::use_onboard::{use_onboard, ManagementForm, ManagementOptions},
};

#[component]
pub fn OnboardingManagement() -> Element {
    let i18 = use_i18();
    let mut onboard = use_onboard();

    rsx!(
        div { class: "form__title",
            span { class: "label",
                {translate!(i18, "onboard.management.label")}
            }
            Title {
                text: translate!(i18, "onboard.management.title")
            }
        }
        div { class: "form__inputs",
            CheckboxCard {
                id: "a".to_string(),
                title: translate!(i18, "onboard.management.form.membership.title"),
                description: translate!(i18, "onboard.management.form.membership.description"),
                name: String::from("management"),
                checked: matches!(onboard.get_management().value, ManagementOptions::Membership ),
                icon: rsx!(
                    Icon {
                        icon: TeamFill,
                        height: 20,
                        width: 20,
                        fill: "var(--fill-600)"
                    }
                ),
                on_change: move |_| {
                    onboard.management_mut().with_mut(|management| management.value =  ManagementOptions::Membership);
                },
            }
            CheckboxCard {
                id: "a".to_string(),
                title: translate!(i18, "onboard.management.form.rank.title"),
                description: translate!(i18, "onboard.management.form.rank.description"),
                name: String::from("management"),
                checked: false,
                soon: true,
                icon: rsx!(
                    Icon {
                        icon: MedalFill,
                        height: 20,
                        width: 20,
                        fill: "var(--fill-600)"
                    }
                ),
                on_change: move |_| {},
            }
            CheckboxCard {
                id: "a".to_string(),
                title: translate!(i18, "onboard.management.form.native_token.title"),
                description: translate!(i18, "onboard.management.form.native_token.description"),
                name: String::from("management"),
                checked: false,
                soon: true,
                icon: rsx!(
                    Icon {
                        icon: KeyFill,
                        height: 20,
                        width: 20,
                        fill: "var(--fill-600)"
                    }
                ),
                on_change: move |_| {},
            }
            CheckboxCard {
                id: "a".to_string(),
                title: translate!(i18, "onboard.management.form.own_token.title"),
                description: translate!(i18, "onboard.management.form.own_token.description"),
                name: String::from("management"),
                checked: false,
                soon: true,
                icon: rsx!(
                    Icon {
                        icon: ShieldKeyholeFill,
                        height: 20,
                        width: 20,
                        fill: "var(--fill-600)"
                    }
                ),
                on_change: move |_| {},
            }
        }
    )
}
