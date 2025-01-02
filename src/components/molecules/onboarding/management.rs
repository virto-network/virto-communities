use dioxus::prelude::*;
use dioxus_i18n::t;
use crate::{
    components::atoms::{
        management_method::ManagementMethod, CheckboxCard, Icon, KeyFill, MedalFill,
        ShieldKeyholeFill, TeamFill, Title,
    },
    hooks::use_onboard::{use_onboard, ManagementOptions},
};
#[component]
pub fn OnboardingManagement() -> Element {
    
    let mut onboard = use_onboard();
    rsx!(
        div { class: "form__title",
            span { class: "label", {t!("onboard-management-label")} }
            Title { text: t!("onboard-management-title") }
        }
        div { class: "form__inputs",
            CheckboxCard {
                id: "a".to_string(),
                name: String::from("management"),
                checked: matches!(onboard.get_management().value, ManagementOptions::Membership),
                body: rsx! {
                    ManagementMethod {
                        title: t!("onboard-management-form-membership-title"),
                        description: t!("onboard-management-form-membership-description"),
                        icon: rsx! {
                            Icon { icon: TeamFill, height: 20, width: 20, fill: "var(--fill-600)" }
                        }
                    }
                },
                on_change: move |_| {
                    onboard
                        .management_mut()
                        .with_mut(|management| management.value = ManagementOptions::Membership);
                }
            }
            CheckboxCard {
                id: "a".to_string(),
                name: String::from("management"),
                checked: false,
                soon: true,
                body: rsx! {
                    ManagementMethod {
                        title: t!("onboard-management-form-rank-title"),
                        description: t!("onboard-management-form-rank-description"),
                        icon: rsx! {
                            Icon { icon: MedalFill, height: 20, width: 20, fill: "var(--fill-600)" }
                        }
                    }
                },
                on_change: move |_| {}
            }
            CheckboxCard {
                id: "a".to_string(),
                name: String::from("management"),
                checked: false,
                soon: true,
                body: rsx! {
                    ManagementMethod {
                        title: t!("onboard-management-form-native_token-title"),
                        description: t!("onboard-management-form-native_token-description"),
                        icon: rsx! {
                            Icon { icon: KeyFill, height: 20, width: 20, fill: "var(--fill-600)" }
                        }
                    }
                },
                on_change: move |_| {}
            }
            CheckboxCard {
                id: "a".to_string(),
                name: String::from("management"),
                checked: false,
                soon: true,
                body: rsx! {
                    ManagementMethod {
                        title: t!("onboard-management-form-own_token-title"),
                        description: t!("onboard-management-form-own_token-description"),
                        icon: rsx! {
                            Icon { icon: ShieldKeyholeFill, height: 20, width: 20, fill: "var(--fill-600)" }
                        }
                    }
                },
                on_change: move |_| {}
            }
        }
    )
}
