use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use futures_util::{StreamExt, TryFutureExt};
use std::str::FromStr;

use crate::{
    components::atoms::{
        dropdown::{DropdownItem, ElementSize},
        icon_button::Variant,
        AccountButton, ArrowUpDown, Button, Close, Dropdown, Hamburguer, Icon, IconButton,
        Messages, Polkadot, Profile, Settings, Votes,
    },
    hooks::{
        use_accounts::{use_accounts, IsDaoOwner},
        use_connect_wallet::{use_connect_wallet, PjsError},
        use_market_client::use_market_client,
        use_notification::use_notification,
        use_session::{use_session, UserSession},
    },
    services::{
        kreivo::{balances::account, communities::is_admin},
        market::types::Tokens,
    },
};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = globalThis, js_name = setSigner)]
    pub fn set_signer(address: String);
}
#[component]
pub fn Header() -> Element {
    let i18 = use_i18();
    let market_client = use_market_client().get();
    let mut accounts = use_accounts();
    let mut notification = use_notification();
    let mut session = use_session();
    let mut ksm_balance = use_signal::<(String, String)>(|| ('0'.to_string(), "00".to_string()));
    let mut usdt_balance = use_signal::<(String, String)>(|| ('0'.to_string(), "00".to_string()));
    let mut is_active = use_signal(|| false);
    let mut connect_handled = use_signal(|| false);

    let get_account = move || {
        let Some(user_session) = session.get() else {
            return None;
        };
        accounts.get_one(user_session.account_id)
    };
    let get_balance = move || {
        spawn({
            let market_client = market_client.to_owned();
            async move {
                let pjs_account =
                    get_account().ok_or(translate!(i18, "errors.wallet.accounts_not_found"))?;
                let account_address = pjs_account.address();
                let address =
                    sp_core::sr25519::Public::from_str(&account_address).map_err(|e| {
                        log::warn!("Not found public address: {}", e);
                        translate!(i18, "errors.wallet.account_address")
                    })?;
                let hex_address = hex::encode(address.0);
                let Ok(account) = account(&format!("0x{}", hex_address)).await else {
                    ksm_balance.set(('0'.to_string(), "00".to_string()));
                    usdt_balance.set(('0'.to_string(), "00".to_string()));
                    return Ok(());
                };
                let is_dao_owner = is_admin(&address.0).await.map_err(|_| {
                    log::warn!("Failed to get is admin");
                    translate!(i18, "errors.wallet.account_address")
                })?;
                accounts.set_is_active_account_an_admin(Some(IsDaoOwner(is_dao_owner)));
                let unscaled_value = account.data.free as f64 / 10_f64.powf(12f64);

                let KSM_PRICE = market_client
                    .get_price_by_token(Tokens::KSM)
                    .await
                    .map_err(|_| String::from("No se ha podido consultar el precio"))?;

                let usdt_value = unscaled_value * KSM_PRICE;
                let usdt_value = usdt_value.to_string();
                let unscaled_value = unscaled_value.to_string();
                let usdt_value = usdt_value.split(".").collect::<Vec<&str>>();
                let unscaled_value = unscaled_value.split(".").collect::<Vec<&str>>();

                ksm_balance.set((
                    unscaled_value[0].to_string(),
                    format!("{:.2}", unscaled_value.get(1).unwrap_or(&"00")),
                ));
                usdt_balance.set((
                    usdt_value[0].to_string(),
                    format!("{:.2}", usdt_value.get(1).unwrap_or(&"00")),
                ));

                Ok::<(), String>(())
            }
            .unwrap_or_else(move |e: String| notification.handle_warning(&e))
        });
    };
    let mut dropdown_value = use_signal::<Option<DropdownItem>>(|| {
        let account = get_account().and_then(|account| {
            Some(DropdownItem {
                key: account.address().clone(),
                value: account.name(),
            })
        });
        account
    });
    let mut items = vec![];
    for account in accounts.get().into_iter() {
        let address = account.address();

        items.push(rsx!(
            AccountButton { title: account.name(), description: address.clone(), on_click: move |_| {} }
        ))
    }

    let on_handle_account = use_coroutine(move |mut rx: UnboundedReceiver<u8>| async move {
        while let Some(event) = rx.next().await {
            let account = &accounts.get()[event as usize];

            let Ok(serialized_session) = serde_json::to_string(&UserSession {
                name: account.name(),
                address: account.address(),
                account_id: event,
            }) else {
                return notification.handle_error(&translate!(i18, "errors.session.persist"));
            };

            if let Err(e) = session.persist_session_file(&serialized_session) {
                log::warn!("Failed to persist session {:?}", e)
            };
            if let Err(e) = session.update_account(event) {
                log::warn!("Failed to update account {:?}", e)
            };

            log::info!("{:?}", account);
            accounts.set_account(Some(account.clone()));
            set_signer(account.address().clone());

            let account = get_account().and_then(|account| {
                Some(DropdownItem {
                    key: account.address().clone(),
                    value: account.name(),
                })
            });

            dropdown_value.set(account);
            get_balance();
        }
    });

    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        if session.is_logged() {
            let Ok(_) = use_connect_wallet().await else {
                return;
            };
            if let Some(user_session) = session.get() {
                on_handle_account.send(user_session.account_id);
            }
        }
    });
    let active_class = if is_active() { "header--active" } else { "" };
    rsx!(
        div { class: "dashboard__header",
            div { class: "dashboard__header__left",
                div { class: "profile" }
                div {
                    class: "welcome",
                    class: if connect_handled() || session.is_logged() { "welcome--positioned" },
                    if session.is_logged() && session.get().is_some() {
                        Dropdown {
                            class: "header__wallet dropdown--left".to_string(),
                            value: dropdown_value(),
                            placeholder: translate!(i18, "header.cta.account"),
                            size: ElementSize::Big,
                            default: None,
                            on_change: move |event: usize| {
                                on_handle_account.send(event as u8);
                            },
                            body: items
                        }
                    } else {
                        if !connect_handled() {
                            Button {
                                text: translate!(i18, "header.cta.connect"),
                                status: None,
                                right_icon: rsx!(
                                    Icon { icon : Polkadot, height : 20, width : 20, fill : "var(--text-primary)" }
                                ),
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
                                value: dropdown_value(),
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
            if session.is_logged() && session.get().is_some() {
                div { class: "dashboard__header__right {active_class}",
                    div { class: "balance__container",
                        div { class: "deposit",
                            IconButton {
                                variant: Variant::Round,
                                size: ElementSize::Big,
                                class: "button--avatar button--comming-soon bg--fill-600",
                                body: rsx!(Icon { icon : ArrowUpDown, height : 32, width : 32, fill : "var(--fill-00)" }),
                                on_click: move |_| {}
                            }
                        }
                        div { class: "balances",
                            span { class: "balance__title",
                                span { class: "balance__value", "{ksm_balance().0}" }
                                span { class: "balance__decimals", ".{ksm_balance().1}" }
                                span { class: "balance__asset", "KSM" }
                            }
                            span { class: "balance__subtitle",
                                span { class: "balance__sign", "$" }
                                span { class: "balance__value", "{usdt_balance().0}" }
                                span { class: "balance__decimals", ".{usdt_balance().1}" }
                                span { class: "balance__asset", "USD" }
                            }
                        }
                    }
                    div { class: "header__cta",
                        IconButton {
                            variant: Variant::Round,
                            size: ElementSize::Big,
                            class: "button--avatar bg--fill-50 button--comming-soon",
                            body: rsx!(Icon { icon : Votes, height : 32, width : 32, fill : "var(--fill-600)" }),
                            on_click: move |_| {}
                        }
                        IconButton {
                            variant: Variant::Round,
                            size: ElementSize::Big,
                            class: "button--avatar bg--fill-50 button--comming-soon",
                            body: rsx!(Icon { icon : Messages, height : 32, width : 32, fill : "var(--fill-600)" }),
                            on_click: move |_| {}
                        }
                        IconButton {
                            variant: Variant::Round,
                            size: ElementSize::Big,
                            class: "button--avatar bg--fill-50 button--comming-soon",
                            body: rsx!(Icon { icon : Profile, height : 32, width : 32, fill : "var(--fill-600)" }),
                            on_click: move |_| {}
                        }
                        IconButton {
                            variant: Variant::Round,
                            size: ElementSize::Big,
                            class: "button--avatar bg--fill-50 button--comming-soon",
                            body: rsx!(Icon { icon : Settings, height : 32, width : 32, fill : "var(--fill-600)" }),
                            on_click: move |_| {}
                        }
                        IconButton {
                            variant: Variant::Round,
                            size: ElementSize::Big,
                            class: "button--avatar bg--fill-50 mobile",
                            body: rsx!(Icon { icon : Hamburguer, height : 32, width : 32, fill : "var(--fill-600)" }),
                            on_click: move |_| {
                                is_active.toggle();
                            }
                        }
                    }
                    div { class: "header__menu mobile",
                        div {
                            class: "header__scream",
                            onclick: move |_| {
                                is_active.toggle();
                            }
                        }
                        div { class: "header__menu__list",
                            div { class: "header__close",
                                IconButton {
                                    variant: Variant::Round,
                                    size: ElementSize::Big,
                                    class: "button--avatar bg--transparent",
                                    body: rsx!(Icon { icon : Close, height : 32, width : 32, fill : "var(--fill-600)" }),
                                    on_click: move |_| {
                                        is_active.toggle();
                                    }
                                }
                            }
                            ul {
                                li {
                                    span { {translate!(i18, "header.menu.invitations")} }
                                    IconButton {
                                        variant: Variant::Round,
                                        size: ElementSize::Big,
                                        class: "button--avatar bg--fill-50 button--comming-soon",
                                        body: rsx!(Icon { icon : Votes, height : 32, width : 32, fill : "var(--fill-600)" }),
                                        on_click: move |_| {}
                                    }
                                }
                                li {
                                    span { {translate!(i18, "header.menu.messages")} }
                                    IconButton {
                                        variant: Variant::Round,
                                        size: ElementSize::Big,
                                        class: "button--avatar bg--fill-50 button--comming-soon",
                                        body: rsx!(Icon { icon : Messages, height : 32, width : 32, fill : "var(--fill-600)" }),
                                        on_click: move |_| {}
                                    }
                                }
                                li {
                                    span { {translate!(i18, "header.menu.profile")} }
                                    IconButton {
                                        variant: Variant::Round,
                                        size: ElementSize::Big,
                                        class: "button--avatar bg--fill-50 button--comming-soon",
                                        body: rsx!(Icon { icon : Profile, height : 32, width : 32, fill : "var(--fill-600)" }),
                                        on_click: move |_| {}
                                    }
                                }
                                li {
                                    span { {translate!(i18, "header.menu.settings")} }
                                    IconButton {
                                        variant: Variant::Round,
                                        size: ElementSize::Big,
                                        class: "button--avatar bg--fill-50 button--comming-soon",
                                        body: rsx!(Icon { icon : Settings, height : 32, width : 32, fill : "var(--fill-600)" }),
                                        on_click: move |_| {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}
