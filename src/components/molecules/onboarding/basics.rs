use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::atoms::{dropdown::ElementSize, notification, Attach, Input, TextareaInput, Title},
    hooks::{
        use_attach::{use_attach, AttachFile},
        use_spaces_client::use_spaces_client,
        use_notification::use_notification,
        use_onboard::{use_onboard, BasicsForm},
    },
};

#[component]
pub fn OnboardingBasics(error: bool) -> Element {
    let i18 = use_i18();
    let mut onboard = use_onboard();
    let mut notification = use_notification();
    let spaces_client = use_spaces_client();

    let mut name_maxlength = use_signal(|| 24);

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
                supported_types: vec![String::from("image/png"), String::from("image/png")],
                on_change: move |event: AttachFile| {
                    spawn(async move {
                        let Ok(uri) = spaces_client.get().upload(event.data, event.name).await else {
                            notification.handle_error(&translate!(i18, "errors.form.upload_fail"));
                            return
                        };
                        onboard.basics_mut().with_mut(|basics| basics.logo = Some(uri))
                    });
                }
            }
            Input {
                message: onboard.get_basics().name,
                size: ElementSize::Big,
                placeholder: translate!(i18, "onboard.basics.form.name.placeholder"),
                error: if error {
                    if onboard.get_basics().name.is_empty() {
                        Some(translate!(i18, "errors.form.not_empty"))
                    } else {
                        None
                    }
                } else { None },
                maxlength: name_maxlength(),
                required: true,
                on_input: move |event: Event<FormData>| {
                    if event.value().as_bytes().len() < 24usize {
                        onboard.basics_mut().with_mut(|basics| basics.name = event.value());
                    } else {
                        name_maxlength.set(event.value().chars().count().try_into().expect("Should convert usize into u8") );
                    }
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
                error: if error {
                    if onboard.get_basics().industry.is_empty() {
                        Some(translate!(i18, "errors.form.not_empty"))
                    } else {
                        None
                    }
                 } else { None },
                required: true,
                on_input: move |event: Event<FormData>| {
                    onboard.basics_mut().with_mut(|basics| basics.industry = event.value());
                },
                on_keypress: move |_| {},
                on_click: move |_| {},
            }
        }
    )
}
