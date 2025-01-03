use dioxus::prelude::*;
use dioxus_i18n::t;
use futures_util::{StreamExt, TryFutureExt};

use crate::{
    components::atoms::{
        button::Variant as ButtonVariant,
        dropdown::{DropdownItem, ElementSize},
        AccountButton, Button, Dropdown, Tab,
    },
    hooks::{
        use_accounts::use_accounts,
        use_market_client::use_market_client,
        use_notification::use_notification,
        use_our_navigator::use_our_navigator,
        use_session::{use_session, UserSession},
        use_timestamp::use_timestamp,
    },
    middlewares::is_chain_available::is_chain_available,
    services::market::types::Tokens,
};
use wasm_bindgen::prelude::*;

#[derive(PartialEq)]
enum AccountTabs {
    Kreivo,
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
    
    let mut notification = use_notification();
    let mut accounts = use_accounts();
    let nav = use_our_navigator();
    let mut session = use_session();
    let timestamp = use_timestamp();
    let market_client = use_market_client().get();
    let mut ksm_balance = use_signal::<(String, String)>(|| ('0'.to_string(), "00".to_string()));
    let mut usdt_balance = use_signal::<(String, String)>(|| ('0'.to_string(), "00".to_string()));

    let kreivo_balance = use_signal(|| 0.0);
    let ksm_usd = use_signal(|| 0.0);

    let tab_value = use_signal(|| AccountTabs::Kreivo);
    let mut profile_value = use_signal(|| ProfileTabs::Wallet);

    let mut dropdown_value = use_signal::<Option<DropdownItem>>(|| None);
    let mut items = vec![];
    for account in accounts.get().into_iter() {
        let address = account.address();

        items.push(rsx!(AccountButton {
            title: account.name(),
            description: address.clone(),
            on_click: move |_| {}
        }))
    }

    let on_account = move || {
        spawn({
            let market_client = market_client.to_owned();
            async move {
                if session.is_logged() {
                    dropdown_value.set(accounts.get_account().map(|account| DropdownItem {
                        key: account.address().clone(),
                        value: account.name(),
                    }));

                    let balance = accounts
                        .get_balance()
                        .await
                        .map_err(|_| t!("warnings-wallet-balance_not_found"))?;

                    let KSM_PRICE = market_client
                        .get_price_by_token(Tokens::KSM)
                        .await
                        .map_err(|_| t!("warnings-market-query_failed"))?;

                    let usdt_value = balance * KSM_PRICE;
                    let usdt_value = usdt_value.to_string();
                    let ksm_value = balance.to_string();
                    let (usdt_units, usdt_decimals) =
                        usdt_value.split_once('.').unwrap_or((&usdt_value, ""));
                    let (ksm_units, ksm_decimals) =
                        ksm_value.split_once('.').unwrap_or((&ksm_value, ""));

                    ksm_balance.set((ksm_units.to_string(), format!("{:.2}", ksm_decimals)));
                    usdt_balance.set((usdt_units.to_string(), format!("{:.2}", usdt_decimals)));
                }
                Ok::<(), String>(())
            }
            .unwrap_or_else(move |e: String| {
                ksm_balance.set(('0'.to_string(), "00".to_string()));
                usdt_balance.set(('0'.to_string(), "00".to_string()));
                notification.handle_warning(&t!("warnings-title"), &e);
            })
        });
    };

    let on_handle_account = use_coroutine(move |mut rx: UnboundedReceiver<()>| {
        let on_account = on_account.clone();
        async move {
            while rx.next().await.is_some() {
                on_account()
            }
        }
    });

    use_effect(use_reactive(
        &accounts.are_accounts_initialized(),
        move |are_accounts_initialized| {
            if are_accounts_initialized {
                on_handle_account.send(())
            }
        },
    ));

    rsx! {
        div { class: "page--vote",
            div { class: "account",
                div { class: "account__balance",
                    div { class: "account__options",
                        Tab {
                            text: t!("account-tabs-wallet-tab"),
                            is_active: matches!(*profile_value.read(), ProfileTabs::Wallet),
                            on_click: move |_| {
                                profile_value.set(ProfileTabs::Wallet);
                            }
                        }
                        Tab {
                            class: "tab--comming-soon",
                            text: t!("account-tabs-transfers-tab"),
                            is_active: true,
                            on_click: move |_| {}
                        }
                    }
                    Dropdown {
                        class: "header__wallet dropdown--right".to_string(),
                        value: dropdown_value(),
                        placeholder: t!("header-cta-account"),
                        size: ElementSize::Medium,
                        default: None,
                        on_change: move |event: usize| {
                            spawn({
                                async move {
                                    let selected_account = accounts.set_account(event).map_err(|_|"errors.session.persist".to_string())?;
                                    on_handle_account.send(());

                                    let _ = session.update_session_file(&UserSession {
                                        name: selected_account.name(),
                                        address: selected_account.address(),
                                        account_id: event as u8,
                                    });
                                    Ok::<(), String>(())
                                }
                                .unwrap_or_else(move |e: String| {
                                    notification.handle_warning(&t!("warnings-title"), &e);
                                })
                            });
                        },
                        body: items
                    }
                }
                match *profile_value.read() {
                    ProfileTabs::Wallet => {
                        rsx!(
                            section { class: "wallet",
                                div { class: "account__container",
                                    div { class: "account__balance",
                                        h3 { class: "account__balance__title",
                                            {t!("account-tabs-wallet-balance-title")}
                                        }
                                        div { class: "account__balance__cta",
                                            Button {
                                                text: t!("account-tabs-wallet-balance-options-deposit"),
                                                size: ElementSize::Small,
                                                variant: ButtonVariant::Secondary,
                                                on_click: move |_| {
                                                    spawn(
                                                        async move {
                                                            nav.push(vec![
                                                                Box::new(is_chain_available(timestamp, notification))
                                                            ], "/deposit");
                                                            Ok::<(), String>(())
                                                        }.unwrap_or_else(move |_: String| {

                                                        })
                                                    );
                                                },
                                                status: None,
                                            }
                                            Button {
                                                text: t!("account-tabs-wallet-balance-options-withdraw"),
                                                size: ElementSize::Small,
                                                variant: ButtonVariant::Secondary,
                                                on_click: move |_| {
                                                    spawn(
                                                        async move {
                                                            nav.push(vec![Box::new(is_chain_available(timestamp, notification))], "/withdraw");
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
                                        {t!("account-tabs-wallet-assets-title")}
                                    }
                                    div { class: "account__actives",
                                        div { class: "actives",
                                            table { class: "actives__list",
                                                tr {
                                                    th { class: "list__name", {t!("account-tabs-wallet-assets-title")} }
                                                    th { {t!("account-tabs-wallet-assets-table-quantity")} }
                                                    th { {t!("account-tabs-wallet-assets-table-cost")} }
                                                    th { {t!("account-tabs-wallet-assets-table-total")} }
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
                                        {t!("account-tabs-transfers-title")}
                                    }
                                    table { class: "actives__list",
                                        tr {
                                            th { class: "list__name", {t!("account-tabs-transfers-table-asset")} }
                                            th { {t!("account-tabs-transfers-table-time")} }
                                            th { {t!("account-tabs-transfers-table-quantity")} }
                                            th { {t!("account-tabs-transfers-table-account")} }
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
