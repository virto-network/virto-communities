use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use futures_util::TryFutureExt;

use crate::{
    components::atoms::{
        button::Variant as ButtonVariant,
        dropdown::{DropdownItem, ElementSize},
        icon_button::Variant,
        AccountButton, ArrowUpDown, Button, Close, Dropdown, Hamburguer, Icon, IconButton,
        Messages, Polkadot, Profile, Settings, Votes,
    },
    hooks::{
        use_accounts::{use_accounts, IsDaoOwner},
        use_connect_wallet::{use_connect_wallet, PjsError},
        use_notification::use_notification,
        use_session::{use_session, UserSession},
    },
    services::kreivo::{balances::account, communities::is_admin},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = globalThis, js_name = setSigner)]
    fn set_signer(address: String);
}

#[component]
pub fn Account() -> Element {
    let i18 = use_i18();
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

                let is_dao_owner = crate::services::kreivo::communities::is_admin(&address.0)
                    .await
                    .map_err(|_| {
                        log::warn!("Failed to get is admin");
                        translate!(i18, "errors.wallet.account_address")
                    })?;

                accounts.set_is_active_account_an_admin(IsDaoOwner(is_dao_owner));

                let unscaled_value = account.data.free as f64 / 10_f64.powf(12f64);
                const KSM_PRICE: f64 = 32.11;

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

        items.push(rsx!(AccountButton {
            title: account.name(),
            description: address.clone(),
            on_click: move |_| {}
        }))
    }

    let mut on_handle_account = move |event: u8| {
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
    };

    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        if session.is_logged() {
            let Ok(_) = use_connect_wallet().await else {
                return;
            };
            if let Some(user_session) = session.get() {
                on_handle_account(user_session.account_id);
            }
        }
    });

    rsx! {
        div { class: "page--vote",
            div { class: "account__container",
                div { class: "account__wrapper",
                    div { class: "account__balance",
                        h3 { class: "account__balance__title",
                            "Balance"
                        }
                        div { class: "account__balance__cta",
                            Button {
                                class: "",
                                text: "Deposit",
                                size: ElementSize::Small,
                                variant: ButtonVariant::Secondary,
                                on_click: move |_| {
                                    spawn(
                                        async move {

                                            Ok::<(), String>(())
                                        }.unwrap_or_else(move |_: String| {

                                        })
                                    );
                                },
                                status: None,
                                left_icon: rsx!(
                                    Icon {
                                        icon: ArrowUpDown,
                                        height: 24,
                                        width: 24,
                                        fill: "var(--fill-600)"
                                    }
                                ),
                            }
                            Button {
                                class: "",
                                text: "Withdraw",
                                size: ElementSize::Small,
                                variant: ButtonVariant::Secondary,
                                on_click: move |_| {
                                    spawn(
                                        async move {

                                            Ok::<(), String>(())
                                        }.unwrap_or_else(move |_: String| {

                                        })
                                    );
                                },
                                status: None,
                                left_icon: rsx!(
                                    Icon {
                                        icon: ArrowUpDown,
                                        height: 24,
                                        width: 24,
                                        fill: "var(--fill-600)"
                                    }
                                ),
                            }
                        }
                    }
                    div { class: "balances",
                        span { class: "balance__title",
                            span { class: "balance__value",
                                "{ksm_balance().0}"
                            }
                            span { class: "balance__decimals",
                                ".{ksm_balance().1}"
                            }
                            span { class: "balance__asset",
                                "KSM"
                            }
                        }
                        span { class: "balance__subtitle",
                            span { class: "balance__sign",
                                "$"
                            }
                            span { class: "balance__value",
                                "{usdt_balance().0}"
                            }
                            span { class: "balance__decimals",
                                ".{usdt_balance().1}"
                            }
                            span { class: "balance__asset",
                                "USD"
                            }
                        }
                    }

                }

                div { class: "account__wrapper",
                    h3 { class: "account__balance__cta",
                        "Activos"
                    }
                    div { class: "account__actives",
                        div { class: "account__actives__cta",
                            Button {
                                class: "",
                                text: "See account on Kreivo",
                                size: ElementSize::Medium,
                                variant: ButtonVariant::Secondary,
                                on_click: move |_| {
                                    spawn(
                                        async move {

                                            Ok::<(), String>(())
                                        }.unwrap_or_else(move |_: String| {

                                        })
                                    );
                                },
                                status: None,
                            }
                            Button {
                                class: "",
                                text: "See account on Kusama",
                                size: ElementSize::Medium,
                                variant: ButtonVariant::Secondary,
                                on_click: move |_| {
                                    spawn(
                                        async move {

                                            Ok::<(), String>(())
                                        }.unwrap_or_else(move |_: String| {

                                        })
                                    );
                                },
                                status: None,
                            }
                        }
                        div { class: "actives",
                            table { class: "actives__list",
                                tr {
                                    th { class: "list__name", "Asset" }
                                    th { "Quantity" }
                                    th { "Cost" }
                                    th { "Total" }
                                }

                                tr {
                                    td { class: "list__name", "KSM" }
                                    td { "10" }
                                    td { "$18.1 USD" }
                                    td { "$181.0 USD" }
                                }

                                tr { class: "list__asset--comming-soon",
                                    td { class: "list__name", "USDT" }
                                    td { "-" }
                                    td { "-" }
                                    td { "-" }
                                }

                                tr { class: "list__asset--comming-soon",
                                    td { class: "list__name", "dUSD" }
                                    td { "-" }
                                    td { "-" }
                                    td { "-" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
