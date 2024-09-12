use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use futures_util::{StreamExt, TryFutureExt};

use crate::{
    components::atoms::{
        dropdown::{DropdownItem, ElementSize},
        AccountButton, BankCardLine, Button, CheckboxCard, Dropdown, Icon, Input, KusamaLogo,
        PaymentMethod, PaypalLogo, PolygonLogo, Tab, Title,
    },
    hooks::{
        use_accounts::use_accounts,
        use_notification::use_notification,
        use_our_navigator::use_our_navigator,
        use_tooltip::{use_tooltip, TooltipItem},
        use_withdraw::use_withdraw,
    },
    pages::onboarding::convert_to_jsvalue,
};
use wasm_bindgen::prelude::*;

pub enum PaymentMethods {
    Card,
    Paypal,
    PSE,
    KUSAMA,
    ETH,
    None,
}

#[derive(PartialEq)]
pub enum WithdrawKreivoTabs {
    Accounts,
    Wallet,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = window, js_name = withdraw)]
    async fn withdrawAction(dest: JsValue, amount: u64) -> Result<JsValue, JsValue>;
}

#[component]
pub fn Withdraw() -> Element {
    let i18 = use_i18();
    let mut withdraw = use_withdraw();

    let mut payment_selected = use_signal(|| PaymentMethods::None);

    let accounts = use_accounts();
    let mut notification = use_notification();
    let mut tooltip = use_tooltip();
    let nav = use_our_navigator();

    let mut tab_value = use_signal(|| WithdrawKreivoTabs::Accounts);

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

    let on_handle_account = use_coroutine(move |mut rx: UnboundedReceiver<u8>| async move {
        while let Some(event) = rx.next().await {
            let account = &accounts.get()[event as usize];

            let value = Some(DropdownItem {
                key: account.address().clone(),
                value: account.name(),
            });

            dropdown_value.set(value);

            withdraw
                .withdraw_mut()
                .with_mut(|w| w.address = account.address())
        }
    });

    use_before_render(move || {
        withdraw.default();
    });

    rsx!(
        div { class: "page--initiative",
            div { class: "payment__form",
                div { class: "form__wrapper",
                    div { class: "form__title",
                        span { class: "label", "Define the withdraw method" }
                        Title { text: "Pick a Method" }
                    }
                    div { class: "row withdraw__row",
                        div { class: "summary summary--form",
                            div { class: "row withdraw__row",
                                div { class: "summary__wrapper",
                                    h4 { class: "summary__subtitle", "Methods" }
                                    div { class: "payment__methods",
                                        CheckboxCard {
                                            id: "a".to_string(),
                                            name: String::from("management"),
                                            checked: matches!(*payment_selected.read(), PaymentMethods::KUSAMA),
                                            class: "checkbox-card--payment",
                                            body: rsx!(
                                                PaymentMethod { title : translate!(i18, "withdraw.payment.methods.kusama.title"),
                                                fee : translate!(i18, "withdraw.payment.methods.kusama.fee"), icon : rsx!(Icon {
                                                icon : KusamaLogo, height : 20, width : 20, fill : "var(--fill-600)" }), }
                                            ),
                                            on_change: move |_| {
                                                payment_selected.set(PaymentMethods::KUSAMA);
                                            }
                                        }
                                        CheckboxCard {
                                            id: "a".to_string(),
                                            name: String::from("management"),
                                            checked: matches!(*payment_selected.read(), PaymentMethods::Card),
                                            soon: true,
                                            class: "checkbox-card--payment",
                                            body: rsx!(
                                                PaymentMethod { title : translate!(i18, "withdraw.payment.methods.card.title"),
                                                fee : translate!(i18, "withdraw.payment.methods.card.fee", fee : 5), icon :
                                                rsx!(Icon { icon : BankCardLine, height : 20, width : 20, fill :
                                                "var(--fill-600)" }), }
                                            ),
                                            on_change: move |_| {
                                                payment_selected.set(PaymentMethods::Card);
                                            }
                                        }
                                        CheckboxCard {
                                            id: "a".to_string(),
                                            name: String::from("management"),
                                            checked: matches!(*payment_selected.read(), PaymentMethods::Paypal),
                                            soon: true,
                                            class: "checkbox-card--payment",
                                            body: rsx!(
                                                PaymentMethod { title : translate!(i18, "withdraw.payment.methods.paypal.title"),
                                                fee : translate!(i18, "withdraw.payment.methods.paypal.fee", fee : 5), icon :
                                                rsx!(Icon { icon : PaypalLogo, height : 20, width : 20, fill : "var(--fill-600)"
                                                }), }
                                            ),
                                            on_change: move |_| {
                                                payment_selected.set(PaymentMethods::Paypal);
                                            }
                                        }
                                        CheckboxCard {
                                            id: "a".to_string(),
                                            name: String::from("management"),
                                            checked: matches!(*payment_selected.read(), PaymentMethods::PSE),
                                            soon: true,
                                            class: "checkbox-card--payment",
                                            body: rsx!(
                                                PaymentMethod { title : translate!(i18, "withdraw.payment.methods.pse.title"),
                                                fee : translate!(i18, "withdraw.payment.methods.pse.fee", fee : 3), icon :
                                                rsx!(Icon { icon : PaypalLogo, height : 20, width : 20, fill : "var(--fill-600)"
                                                }), }
                                            ),
                                            on_change: move |_| {
                                                payment_selected.set(PaymentMethods::PSE);
                                            }
                                        }
                                        CheckboxCard {
                                            id: "a".to_string(),
                                            name: String::from("management"),
                                            checked: matches!(*payment_selected.read(), PaymentMethods::ETH),
                                            soon: true,
                                            class: "checkbox-card--payment",
                                            body: rsx!(
                                                PaymentMethod { title : translate!(i18, "withdraw.payment.methods.eth.title"),
                                                fee : translate!(i18, "withdraw.payment.methods.eth.fee"), icon : rsx!(Icon {
                                                icon : PolygonLogo, height : 20, width : 20, fill : "var(--fill-600)" }), }
                                            ),
                                            on_change: move |_| {
                                                payment_selected.set(PaymentMethods::ETH);
                                            }
                                        }
                                    }
                                }
                                if !matches!(*payment_selected.read(), PaymentMethods::None) {
                                    div { class: "summary__wrapper",
                                        h4 { class: "summary__subtitle", "Withdraw Address" }
                                        div { class: "withdraw__form__inputs",
                                            div { class: "account__options",
                                                Tab {
                                                    text: "My accounts",
                                                    is_active: if *tab_value.read() == WithdrawKreivoTabs::Accounts { true } else { false },
                                                    on_click: move |_| {
                                                        tab_value.set(WithdrawKreivoTabs::Accounts);
                                                    }
                                                }
                                                Tab {
                                                    text: "Others",
                                                    is_active: if *tab_value.read() == WithdrawKreivoTabs::Wallet { true } else { false },
                                                    on_click: move |_| {
                                                        tab_value.set(WithdrawKreivoTabs::Wallet);
                                                    }
                                                }
                                            }
                                            div { class: "widthdraw__data",
                                                match *tab_value.read() {
                                                    WithdrawKreivoTabs::Accounts => rsx!{
                                                        Dropdown {
                                                            class: "payment__wallet dropdown--left".to_string(),
                                                            value: dropdown_value(),
                                                            label: "Account",
                                                            size: ElementSize::Medium,
                                                            placeholder: translate!(i18, "header.cta.account"),
                                                            default: None,
                                                            on_change: move |event: usize| {
                                                                on_handle_account.send(event as u8);
                                                            },
                                                            body: items
                                                        }
                                                    },
                                                    WithdrawKreivoTabs::Wallet => rsx!{
                                                        Input {
                                                            message: withdraw.get_withdraw().address,
                                                            placeholder: "5HBVkGX...",
                                                            label: "Address",
                                                            error: None,
                                                            on_input: move |event: Event<FormData>| {
                                                                dropdown_value.set(None);
                                                                withdraw
                                                                    .withdraw_mut()
                                                                    .with_mut(|w| w.address = event.value());
                                                            },
                                                            on_keypress: move |_| {},
                                                            on_click: move |_| {},
                                                        }
                                                    },
                                                },
                                                Input {
                                                    message: withdraw.get_withdraw().amount,
                                                    placeholder: "Amount",
                                                    label: "Amount",
                                                    error: None,
                                                    right_text: rsx!(span { class : "input--right__text", "KSM" }),
                                                    on_input: move |event: Event<FormData>| {
                                                        withdraw
                                                            .withdraw_mut()
                                                            .with_mut(|w| {
                                                                w.amount = event.value();
                                                            })
                                                    },
                                                    on_keypress: move |_| {},
                                                    on_click: move |_| {}
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "summary",
                            h4 { class: "summary__subtitle", "Description" }
                            hr { class: "divider" }
                            section { class: "summary__item summary__item--description",
                                span { class: "summary__item__label--subtitle",
                                    "From Kreivo - To Kusama"
                                }
                                p { class: "summary__item__description",
                                    "You are going to withdraw your KSM from Kreivo to Kusama"
                                }
                            }

                            Button {
                                text: "Confirm withdraw",
                                disabled: !withdraw.is_form_complete(),
                                size: ElementSize::Medium,
                                on_click: move |_| {
                                    spawn(
                                        async move {
                                            tooltip
                                                .handle_tooltip(TooltipItem {
                                                    title: "Withdraw in course".to_string(),
                                                    body: "This could take a moment".to_string(),
                                                    show: true,
                                                });
                                            let address = sp_core::sr25519::Public::from_str(
                                                    &withdraw.get_withdraw().address,
                                                )
                                                .map_err(|e| {
                                                    log::warn!("Not found public address: {}", e);
                                                    translate!(i18, "errors.wallet.account_address")
                                                })?;
                                            let hex_address = hex::encode(address.0);
                                            let destination_address = convert_to_jsvalue(
                                                    &format!("0x{}", hex_address),
                                                )
                                                .map_err(|_| {
                                                    log::warn!("Malformed dest account");
                                                    String::from("Invalid address destination")
                                                })?;
                                            let amount = withdraw
                                                .get_withdraw()
                                                .amount
                                                .parse::<f64>()
                                                .map_err(|_| {
                                                    log::warn!("Malformed amount");
                                                    String::from("Invalid amount to withdraw")
                                                })?;
                                            let amount = (amount * 1_000_000_000_000.0) as u64;
                                            withdrawAction(destination_address, amount)
                                                .await
                                                .map_err(|e| {
                                                    log::warn!("Withdraw failed {:?}", e);
                                                    String::from("Withdraw Failed")
                                                })?;
                                            tooltip.hide();
                                            notification.handle_success("Your withdraw was completed");
                                            nav.push(vec![], "/account");
                                            Ok::<(), String>(())
                                        }
                                            .unwrap_or_else(move |e: String| {
                                                tooltip.hide();
                                                notification.handle_error(&e);
                                            }),
                                    );
                                },
                                status: None
                            }
                        }
                    }
                }
            }
        }
    )
}
