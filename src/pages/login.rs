use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::atoms::{
        button::Variant, dropdown::ElementSize, input::InputType, Button, CircleCheck, Icon, Input,
        Polkadot, VirtoLogo,
    },
    hooks::{
        use_connect_wallet::{use_connect_wallet, PjsError},
        use_notification::use_notification,
    },
};
use futures_util::TryFutureExt;

#[component]
pub fn Login() -> Element {
    let i18 = use_i18();
    let mut notification = use_notification();

    rsx! {
        div { class: "page page--onboarding",
            div { class: "login",
                div { class: "login__container",
                    div { class: "login__info__wrapper",
                        div { class: "login__head",
                            Icon {
                                icon: VirtoLogo,
                                height: 64,
                                width: 64,
                                stroke_width: 1,
                                fill: "var(--color-lavanda-400)"
                            }
                            div { class: "login__welcome",
                                "Welcome to"
                            }
                            div { class: "login__name",
                                "VIRTO"
                            }
                        }
                        div { class: "login__info",
                            p { class: "login__info__description",
                                "Explore local projects and Initiatives that are making an impact"
                            }
                            ul { class: "login__info__opportunities",
                                li { class: "icon-text",
                                    Icon {
                                        icon: CircleCheck,
                                        height: 36,
                                        width: 36,
                                        fill: "var(--state-primary-active)"
                                    }
                                    span { class: "icon-text__title",
                                        "Connect with others around you"
                                    }
                                }
                                li { class: "icon-text",
                                    Icon {
                                        icon: CircleCheck,
                                        height: 36,
                                        width: 36,
                                        fill: "var(--state-primary-active)"
                                    }
                                    span { class: "icon-text__title",
                                        "Show your work, learn from others"
                                    }
                                }
                                li { class: "icon-text",
                                    Icon {
                                        icon: CircleCheck,
                                        height: 36,
                                        width: 36,
                                        fill: "var(--state-primary-active)"
                                    }
                                    span { class: "icon-text__title",
                                        "Find new opportunities to make an impact."
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "login__form",
                    div { class: "login__form__wrapper",
                        div { class: "login__form__head",
                            h3 { class: "login__form__title",
                                "Login"
                            },
                            p { class: "login__form__description",
                                "Sign in with your email address or phone"
                            }
                        }
                        Input {
                            message: "".to_string(),
                            size: ElementSize::Big,
                            label: "Email or Username",
                            placeholder: "Enter your email or username",
                            error: None,
                            required: true,
                            on_input: move |_: Event<FormData>| {

                            },
                            on_keypress: move |_| {},
                            on_click: move |_| {},
                        }
                        Input {
                            message: "".to_string(),
                            size: ElementSize::Big,
                            itype: InputType::Password,
                            label: "Password",
                            placeholder: "Enter your password",
                            error: None,
                            required: true,
                            on_input: move |_: Event<FormData>| {

                            },
                            on_keypress: move |_| {},
                            on_click: move |_| {},
                        }
                        div { class: "login__form__cta",
                            Button {
                                class: "",
                                text: "Continue",
                                size: ElementSize::Medium,
                                on_click: move |_| {

                                },
                                status: None,
                            }
                            Button {
                                class: "",
                                text: "Continue with PolkadotJS",
                                size: ElementSize::Medium,
                                variant: Variant::Secondary,
                                on_click: move |_| {
                                    spawn(
                                        async move {
                                            use_connect_wallet().await?;

                                            Ok::<(), PjsError>(())
                                        }.unwrap_or_else(move |e: PjsError| {
                                            match e {
                                                PjsError::ConnectionFailed => {
                                                    notification.handle_error(&translate!(i18, "errors.wallet.connection_failed"))
                                                }
                                                PjsError::AccountsNotFound => {
                                                    notification.handle_error(&translate!(i18, "errors.wallet.accounts_not_found"));
                                                }
                                            };
                                        })
                                    );
                                },
                                status: None,
                                left_icon: rsx!(
                                    Icon {
                                        icon: Polkadot,
                                        height: 24,
                                        width: 24,
                                        fill: "var(--fill-600)"
                                    }
                                ),
                            }
                        }
                    }
                }
            }
        }
    }
}
