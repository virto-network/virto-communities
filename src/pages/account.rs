use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use futures_util::TryFutureExt;

use crate::{
    components::atoms::{button::Variant as ButtonVariant, dropdown::ElementSize, Button, Tab},
    hooks::{
        use_market_client::use_market_client, use_notification::use_notification, use_our_navigator::use_our_navigator, use_session::use_session
    },
    services::{kreivo, kusama, market::types::Tokens},
};
use wasm_bindgen::prelude::*;

#[derive(PartialEq)]
enum AccountTabs {
    Kreivo,
    Kusama,
}

#[derive(PartialEq)]
enum ProfileTabs {
    Wallet,
    _Transfers,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = globalThis, js_name = setSigner)]
    fn set_signer(address: String);
}

#[component]
pub fn Account() -> Element {
    let i18 = use_i18();
    let mut notification = use_notification();
    let nav = use_our_navigator();
    let session = use_session();
    let market_client = use_market_client().get();
    let mut ksm_balance = use_signal::<(String, String)>(|| ('0'.to_string(), "00".to_string()));
    let mut usdt_balance = use_signal::<(String, String)>(|| ('0'.to_string(), "00".to_string()));

    let mut kreivo_balance = use_signal(|| 0.0);
    let mut kusama_balance = use_signal(|| 0.0);
    let mut ksm_usd = use_signal(|| 0.0);

    let get_balance = move || {
        spawn({
            async move {
                let user_session = session
                    .get()
                    .ok_or(translate!(i18, "errors.wallet.accounts_not_found"))?;

                let address =
                    sp_core::sr25519::Public::from_str(&user_session.address).map_err(|e| {
                        log::warn!("Not found public address: {}", e);
                        translate!(i18, "errors.wallet.account_address")
                    })?;

                let hex_address = hex::encode(address.0);

                let Ok(account) = kreivo::balances::account(&format!("0x{}", hex_address)).await
                else {
                    ksm_balance.set(('0'.to_string(), "00".to_string()));
                    usdt_balance.set(('0'.to_string(), "00".to_string()));

                    return Ok(());
                };

                kreivo_balance.set(account.data.free as f64 / 10_f64.powf(12f64));

                let Ok(account) = kusama::balances::account(&format!("0x{}", hex_address)).await
                else {
                    ksm_balance.set(('0'.to_string(), "00".to_string()));
                    usdt_balance.set(('0'.to_string(), "00".to_string()));

                    return Ok(());
                };

                kusama_balance.set(account.data.free as f64 / 10_f64.powf(12f64));

                let unscaled_value = kreivo_balance() + kusama_balance();
                let KSM_PRICE = market_client
                    .get_price_by_token(Tokens::KSM)
                    .await
                    .map_err(|_| String::from("No se ha podido consultar el precio"))?;

                ksm_usd.set(KSM_PRICE);

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

    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        if session.is_logged() {
            get_balance();
        }
    });

    let mut tab_value = use_signal(|| AccountTabs::Kreivo);
    let mut profile_value = use_signal(|| ProfileTabs::Wallet);

    rsx! {
        div { class: "page--vote",
            div { class: "account",
                div { class: "account__options",
                    Tab {
                        text: "Wallet",
                        is_active: if *profile_value.read() == ProfileTabs::Wallet { true } else {false},
                        on_click: move |_| {
                            profile_value.set(ProfileTabs::Wallet);
                        },
                    }
                    Tab {
                        class: "tab--comming-soon",
                        text: "Transfers",
                        is_active: true,
                        on_click: move |_| {},
                    }
                }
                match *profile_value.read() {
                    ProfileTabs::Wallet => {
                        rsx!(
                            section { class: "wallet",
                                div { class: "account__container",
                                    div { class: "account__balance",
                                        h3 { class: "account__balance__title",
                                            "Balance"
                                        }
                                        div { class: "account__balance__cta",
                                            Button {
                                                class: "button--comming-soon",
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
                                            }
                                            Button {
                                                class: "",
                                                text: "Withdraw",
                                                size: ElementSize::Small,
                                                variant: ButtonVariant::Secondary,
                                                on_click: move |_| {
                                                    spawn(
                                                        async move {
                                                            nav.push(vec![], "/withdraw");
                                                            Ok::<(), String>(())
                                                        }.unwrap_or_else(move |_: String| {

                                                        })
                                                    );
                                                },
                                                status: None,
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
                                div { class: "account__container",
                                    h3 { class: "account__balance__title",
                                        "Activos"
                                    }
                                    div { class: "account__actives",
                                        div { class: "account__actives__cta",
                                            Tab {
                                                text: "See account on Kreivo",
                                                is_active: if *tab_value.read() == AccountTabs::Kreivo { true } else {false},
                                                on_click: move |_| {
                                                    tab_value.set(AccountTabs::Kreivo);
                                                },
                                            }
                                            Tab {
                                                text: "See account on Kusama",
                                                is_active: if *tab_value.read() == AccountTabs::Kusama { true } else {false},
                                                on_click: move |_| {
                                                    tab_value.set(AccountTabs::Kusama);
                                                },
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

                                                match *tab_value.read() {
                                                    AccountTabs::Kreivo => rsx!(
                                                        tr {
                                                            td { class: "list__name", "KSM" }
                                                            td { { format!("{:.2}", kreivo_balance()) } }
                                                            td {
                                                                { format!("${} USD", if ksm_usd() == 0.0 { "-".to_string() } else { format!("{:.2}", ksm_usd()) } )}
                                                            }
                                                            td {
                                                                { format!("${} USD", if ksm_usd() == 0.0 || kreivo_balance() <= 0.001  { "-".to_string() } else { format!("{:.2}", ksm_usd() * kreivo_balance()) } )}
                                                            }
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
                                                    ),
                                                    AccountTabs::Kusama => rsx!(
                                                        tr {
                                                            td { class: "list__name", "KSM" }
                                                            td { { format!("{:.2}", kusama_balance()) } }
                                                            td {
                                                                { format!("${} USD", if ksm_usd() == 0.0 { "-".to_string() } else { format!("{:.2}", ksm_usd()) } )}
                                                            }
                                                            td {
                                                                { format!("${} USD", if ksm_usd() == 0.0 || kusama_balance() <= 0.001  { "-".to_string() } else { format!("{:.2}", ksm_usd() * kusama_balance()) } )}
                                                            }
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
                                                    ),
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        )
                    },
                    ProfileTabs::_Transfers => {
                        rsx!(
                            section { class: "transfers",
                                div { class: "account__container",
                                    h3 { class: "account__balance__title",
                                        "Transferencias"
                                    }
                                    table { class: "actives__list",
                                        tr {
                                            th { class: "list__name", "Asset" }
                                            th { "Time" }
                                            th { "Quantity" }
                                            th { "Account" }
                                        }

                                        tr {
                                            td { class: "list__name", "KSM" }
                                            td { "2024-08-20 20:16:34" }
                                            td { "10" }
                                            td { "5E4S9C..." }
                                        }

                                        tr {
                                            td { class: "list__name", "KSM" }
                                            td { "2024-08-20 20:16:34" }
                                            td { "10" }
                                            td { "5E4S9C..." }
                                        }

                                        tr {
                                            td { class: "list__name", "KSM" }
                                            td { "2024-08-20 20:16:34" }
                                            td { "10" }
                                            td { "5E4S9C..." }
                                        }

                                        tr {
                                            td { class: "list__name", "KSM" }
                                            td { "2024-08-20 20:16:34" }
                                            td { "10" }
                                            td { "5E4S9C..." }
                                        }
                                    }
                                }
                            }
                        )
                    },
                }
            }
        }
    }
}
