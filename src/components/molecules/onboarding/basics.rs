use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::atoms::{dropdown::ElementSize, Attach, Input, TextareaInput, Title},
    hooks::use_onboard::use_onboard,
};

#[component]
pub fn OnboardingBasics() -> Element {
    let i18 = use_i18();
    let mut onboard = use_onboard();

    rsx!(
        div { class: "form__title",
            span { class: "label",
                {translate!(i18, "onboard.basics.label")}
            }
            Title {
                text: translate!(i18, "onboard.basics.title")
            }
        }
        div { class: "form__inputs",
            Attach {
                cta_text: translate!(i18, "onboard.basics.form.logo.placeholder"),
                on_change: move |_| {}
            }
            Input {
                message: onboard.get_basics().name,
                size: ElementSize::Big,
                placeholder: translate!(i18, "onboard.basics.form.name.placeholder"),
                error: None,
                on_input: move |event: Event<FormData>| {
                    onboard.basics_mut().with_mut(|basics| basics.name = event.value());
                },
                on_keypress: move |_| {},
                on_click: move |_| {},
            }
            TextareaInput {
                value: onboard.get_basics().description,
                placeholder: translate!(i18, "onboard.basics.form.description.placeholder"),
                on_input: move |event: Event<FormData>| {
                    onboard.basics_mut().with_mut(|basics| basics.description = event.value());
                },
                on_keypress: move |_| {},
                on_click: move |_| {},
            }
            Input {
                message: onboard.get_basics().industry,
                size: ElementSize::Big,
                placeholder: translate!(i18, "onboard.basics.form.industry.placeholder"),
                error: None,
                on_input: move |event: Event<FormData>| {
                    onboard.basics_mut().with_mut(|basics| basics.industry = event.value());
                },
                on_keypress: move |_| {},
                on_click: move |_| {},
            }
        }
    )
}
