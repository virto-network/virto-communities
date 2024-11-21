use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::atoms::{
        button::Variant, dropdown::ElementSize, AccountButton, Button, CircleCheck, Dropdown, Icon,
        Polkadot, VirtoLogo,
    },
    hooks::{
        use_accounts::use_accounts,
        use_connect_wallet::{use_connect_wallet, PjsError},
        use_notification::use_notification,
        use_our_navigator::use_our_navigator,
        use_session::{use_session, UserSession},
    },
};
use futures_util::{StreamExt, TryFutureExt};

#[component]
pub fn Login() -> Element {
    let i18 = use_i18();
    let mut notification = use_notification();
    let accounts = use_accounts();
    let mut session = use_session();
    let mut connect_handled = use_signal(|| false);
    let nav = use_our_navigator();

    let mut items = vec![];
    for account in accounts.get().into_iter() {
        let address = account.address();

        items.push(rsx!(AccountButton {
            title: account.name(),
            description: address.clone(),
            on_click: move |_| {}
        }))
    }

    let on_handle_account = use_coroutine(move |mut rx: UnboundedReceiver<u8>| async move {
        while let Some(event) = rx.next().await {
            let Some(selected_account) = &accounts.get_one(event as usize) else {
                return notification.handle_warning(
                    &translate!(i18, "warnings.title"),
                    &translate!(i18, "warnings.middleware.not_account"),
                );
            };

            let Ok(_) = session.update_session_file(&UserSession {
                name: selected_account.name(),
                address: selected_account.address(),
                account_id: event as u8,
            }) else {
                return notification.handle_error(&translate!(i18, "errors.session.persist"));
            };

            nav.push(vec![], "/");
        }
    });

    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        if session.get().is_some() {
            nav.push(vec![], "/")
        }
    });

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
                            div { class: "login__welcome", {translate!(i18, "login.welcome")} }
                            div { class: "login__name", "VIRTO" }
                        }
                        div { class: "login__info",
                            p { class: "login__info__description",
                                {translate!(i18, "login.description")}
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
                                        {translate!(i18, "login.opportunities.connect")}
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
                                        {translate!(i18, "login.opportunities.learn")}
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
                                        {translate!(i18, "login.opportunities.impact")}
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "login__form",
                    div { class: "login__form__wrapper",
                        div { class: "login__form__head",
                            h3 { class: "login__form__title", {translate!(i18, "login.form.title")} }
                        }
                        div { class: "login__form__cta",
                            if !connect_handled() {
                                Button {
                                    text: translate!(i18, "header.cta.connect"),
                                    status: None,
                                    variant: Variant::Secondary,
                                    right_icon: rsx! {
                                        Icon { icon: Polkadot, height: 20, width: 20, fill: "var(--text-primary)" }
                                    },
                                    on_click: move |_| {
                                        spawn(
                                            async move {
                                                use_connect_wallet().await?;
                                                connect_handled.toggle();
                                                Ok::<(), PjsError>(())
                                            }
                                                .unwrap_or_else(move |e: PjsError| {
                                                    match e {
                                                        PjsError::ConnectionFailed => {
                                                            notification
                                                                .handle_error(
                                                                    &translate!(i18, "errors.wallet.connection_failed"),
                                                                )
                                                        }
                                                        PjsError::AccountsNotFound => {
                                                            notification
                                                                .handle_error(
                                                                    &translate!(i18, "errors.wallet.accounts_not_found"),
                                                                );
                                                        }
                                                    };
                                                }),
                                        );
                                    }
                                }
                            } else {
                                Dropdown {
                                    class: "header__wallet dropdown--left".to_string(),
                                    value: None,
                                    size: ElementSize::Big,
                                    placeholder: translate!(i18, "header.cta.account"),
                                    default: None,
                                    on_change: move |event: usize| {
                                        on_handle_account.send(event as u8);
                                    },
                                    body: items
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
