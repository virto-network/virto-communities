use dioxus::prelude::*;
use dioxus_i18n::t;
use crate::{
    components::atoms::{dropdown::ElementSize, Attach, Input, TextareaInput, Title},
    hooks::{
        use_attach::AttachFile, use_notification::use_notification,
        use_onboard::use_onboard, use_spaces_client::use_spaces_client,
    },
};
#[component]
pub fn OnboardingBasics(error: bool) -> Element {
    
    let mut onboard = use_onboard();
    let mut notification = use_notification();
    let spaces_client = use_spaces_client();
    let mut name_maxlength = use_signal(|| 24);
    rsx!(
        div { class: "form__title",
            span { class: "label", {t!("onboard-basics-label")} }
            Title { text: t!("onboard-basics-title") }
        }
        div { class: "form__inputs",
            Attach {
                cta_text: t!("onboard-basics-form-logo-placeholder"),
                supported_types: vec![String::from("image/png"), String::from("image/png")],
                on_change: move |event: AttachFile| {
                    spawn(async move {
                        let Ok(uri) = spaces_client.get().upload(&event.data, &event.name).await
                        else {
                            notification.handle_error(&t!("errors-form-upload_fail"));
                            return;
                        };
                        onboard.basics_mut().with_mut(|basics| basics.logo = Some(uri))
                    });
                }
            }
            Input {
                message: onboard.get_basics().name,
                size: ElementSize::Big,
                placeholder: t!("onboard-basics-form-name-placeholder"),
                error: if error {
                    if onboard.get_basics().name.is_empty() {
                        Some(t!("errors-form-not_empty"))
                    } else {
                        None
                    }
                } else {
                    None
                },
                maxlength: name_maxlength(),
                required: true,
                on_input: move |event: Event<FormData>| {
                    if event.value().as_bytes().len() < 24usize {
                        onboard.basics_mut().with_mut(|basics| basics.name = event.value());
                    } else {
                        name_maxlength
                            .set(
                                event
                                    .value()
                                    .chars()
                                    .count()
                                    .try_into()
                                    .expect("Should convert usize into u8"),
                            );
                    }
                },
                on_keypress: move |_| {},
                on_click: move |_| {}
            }
            TextareaInput {
                value: onboard.get_basics().description,
                placeholder: t!("onboard-basics-form-description-placeholder"),
                on_input: move |event: Event<FormData>| {
                    onboard.basics_mut().with_mut(|basics| basics.description = event.value());
                },
                on_keypress: move |_| {},
                on_click: move |_| {}
            }
            Input {
                message: onboard.get_basics().industry,
                size: ElementSize::Big,
                placeholder: t!("onboard-basics-form-industry-placeholder"),
                error: if error {
                    if onboard.get_basics().industry.is_empty() {
                        Some(t!("errors-form-not_empty"))
                    } else {
                        None
                    }
                } else {
                    None
                },
                required: true,
                on_input: move |event: Event<FormData>| {
                    onboard.basics_mut().with_mut(|basics| basics.industry = event.value());
                },
                on_keypress: move |_| {},
                on_click: move |_| {}
            }
        }
    )
}
