use dioxus::{logger::tracing::{debug, warn, info}, prelude::*};
use dioxus_i18n::t;
use futures_util::{StreamExt, TryFutureExt};

use crate::{
    components::atoms::{
        dropdown::{DropdownItem, ElementSize},
        AccountButton, BankCardLine, Button, CheckboxCard, Dropdown, Icon, Input, KusamaLogo,
        PaymentMethod, PaypalLogo, PolygonLogo, Tab, Title,
    }, hooks::{
        use_accounts::use_accounts,
        use_communities::use_communities,
        use_deposit::{use_deposit, DepositError, DepositTo},
        use_notification::use_notification,
        use_our_navigator::use_our_navigator,
        use_tooltip::{use_tooltip, TooltipItem},
    }, middlewares::is_signer_ready::is_signer_ready, pages::onboarding::convert_to_jsvalue
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

#[derive(PartialEq, Clone)]
pub enum DepositKreivoTabs {
    Accounts,
    Wallet,
    Community,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = window, js_name = deposit)]
    async fn depositAction(
        dest: JsValue,
        amount: u64,
        to_community: bool,
    ) -> Result<JsValue, JsValue>;
}

#[component]
pub fn Deposit() -> Element {
    
    let mut deposit = use_deposit();

    let accounts = use_accounts();
    let communities = use_communities();
    let mut notification = use_notification();
    let mut tooltip = use_tooltip();
    let nav = use_our_navigator();

    let mut payment_selected = use_signal(|| PaymentMethods::None);
    let mut tab_value = use_signal(|| DepositKreivoTabs::Accounts);

    let mut dropdown_value = use_signal::<Option<DropdownItem>>(|| None);

    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        if is_signer_ready(accounts, notification)().is_err() {
            nav.push(vec![], "/login");
        };
    });
    
    let mut items = vec![];
    for account in accounts.get().into_iter() {
        let address = account.address();

        items.push(rsx!(
            AccountButton { title: account.name(), description: address.clone(), on_click: move |_| {} }
        ))
    }

    let mut community_items = vec![];
    for community in communities.get_communities().into_iter() {
        community_items.push(rsx!(
            AccountButton {
                title: format!("{} ({})", community.name, community.id),
                description: "".to_string(),
                on_click: move |_| {}
            }
        ))
    }

    let on_handle_account = use_coroutine(move |mut rx: UnboundedReceiver<u8>| async move {
        while let Some(event) = rx.next().await {
            let account = &accounts.get()[event as usize];

            let value = Some(DropdownItem {
                key: account.address().clone(),
                value: account.name(),
            });

            dropdown_value.set(value);

            deposit
                .deposit_mut()
                .with_mut(|w| w.dest = DepositTo::Address(account.address()))
        }
    });

    let on_handle_community = use_coroutine(move |mut rx: UnboundedReceiver<u8>| async move {
        while let Some(event) = rx.next().await {
            let community = &communities.get_communities()[event as usize];

            let value = Some(DropdownItem {
                key: community.id.to_string(),
                value: community.name.clone(),
            });

            dropdown_value.set(value);

            deposit
                .deposit_mut()
                .with_mut(|w| w.dest = DepositTo::Community(community.id))
        }
    });

    use_effect(use_reactive((&*tab_value.read(),), move |(_,)| {
        dropdown_value.set(None);
    }));

    use_before_render(move || {
        deposit.default();
    });

    rsx!(
        div { class: "page--initiative",
            div { class: "payment__form",
                div { class: "form__wrapper",
                    div { class: "form__title",
                        span { class: "label", {t!("deposit-payment-label")} }
                        Title { text: t!("deposit-payment-title") }
                    }
                    div { class: "row deposit__row",
                        div { class: "summary summary--form",
                            div { class: "row deposit__row",
                                div { class: "summary__wrapper",
                                    h4 { class: "summary__subtitle",
                                        {t!("deposit-payment-subtitle")}
                                    }
                                    div { class: "payment__methods",
                                        CheckboxCard {
                                            id: "a".to_string(),
                                            name: String::from("management"),
                                            checked: matches!(*payment_selected.read(), PaymentMethods::KUSAMA),
                                            class: "checkbox-card--payment",
                                            body: rsx! {
                                                PaymentMethod {
                                                    title: t!("deposit-payment-methods-kusama-title"),
                                                    fee: t!("deposit-payment-methods-kusama-fee"),
                                                    icon: rsx! {
                                                        Icon { icon: KusamaLogo, height: 20, width: 20, fill: "var(--fill-600)" }
                                                    }
                                                }
                                            },
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
                                            body: rsx! {
                                                PaymentMethod {
                                                    title: t!("deposit-payment-methods-card-title"),
                                                    fee: t!("deposit-payment-methods-card-fee", fee : 5),
                                                    icon: rsx! {
                                                        Icon { icon: BankCardLine, height: 20, width: 20, fill: "var(--fill-600)" }
                                                    }
                                                }
                                            },
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
                                            body: rsx! {
                                                PaymentMethod {
                                                    title: t!("deposit-payment-methods-paypal-title"),
                                                    fee: t!("deposit-payment-methods-paypal-fee", fee : 5),
                                                    icon: rsx! {
                                                        Icon { icon: PaypalLogo, height: 20, width: 20, fill: "var(--fill-600)" }
                                                    }
                                                }
                                            },
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
                                            body: rsx! {
                                                PaymentMethod {
                                                    title: t!("deposit-payment-methods-pse-title"),
                                                    fee: t!("deposit-payment-methods-pse-fee", fee : 3),
                                                    icon: rsx! {
                                                        Icon { icon: PaypalLogo, height: 20, width: 20, fill: "var(--fill-600)" }
                                                    }
                                                }
                                            },
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
                                            body: rsx! {
                                                PaymentMethod {
                                                    title: t!("deposit-payment-methods-eth-title"),
                                                    fee: t!("deposit-payment-methods-eth-fee"),
                                                    icon: rsx! {
                                                        Icon { icon: PolygonLogo, height: 20, width: 20, fill: "var(--fill-600)" }
                                                    }
                                                }
                                            },
                                            on_change: move |_| {
                                                payment_selected.set(PaymentMethods::ETH);
                                            }
                                        }
                                    }
                                }
                                if !matches!(*payment_selected.read(), PaymentMethods::None) {
                                    div { class: "summary__wrapper",
                                        h4 { class: "summary__subtitle",
                                            {t!("deposit-form-title")}
                                        }
                                        div { class: "deposit__form__inputs",
                                            div { class: "account__options",
                                                Tab {
                                                    text: t!("deposit-tabs-accounts"),
                                                    is_active: matches!(*tab_value.read(), DepositKreivoTabs::Accounts),
                                                    on_click: move |_| {
                                                        tab_value.set(DepositKreivoTabs::Accounts);
                                                    }
                                                }
                                                Tab {
                                                    text: t!("deposit-tabs-others"),
                                                    is_active: matches!(*tab_value.read(), DepositKreivoTabs::Wallet),
                                                    on_click: move |_| {
                                                        tab_value.set(DepositKreivoTabs::Wallet);
                                                    }
                                                }
                                                Tab {
                                                    text: t!("deposit-tabs-communities"),
                                                    is_active: matches!(*tab_value.read(), DepositKreivoTabs::Community),
                                                    on_click: move |_| {
                                                        tab_value.set(DepositKreivoTabs::Community);
                                                    }
                                                }
                                            }
                                            div { class: "widthdraw__data",
                                                match *tab_value.read() {
                                                    DepositKreivoTabs::Accounts => rsx!{
                                                        Dropdown {
                                                            class: "payment__wallet dropdown--left".to_string(),
                                                            value: dropdown_value(),
                                                            label: t!("deposit-form-account-label"),
                                                            size: ElementSize::Medium,
                                                            placeholder: t!("header-cta-account"),
                                                            default: None,
                                                            on_change: move |event: usize| {
                                                                on_handle_account.send(event as u8);
                                                            },
                                                            body: items
                                                        }
                                                    },
                                                    DepositKreivoTabs::Wallet => rsx!{
                                                        Input {
                                                            message: deposit.get_deposit().address(),
                                                            placeholder: "5HBVkGX...",
                                                            label: t!("deposit-form-address-label"),
                                                            error: None,
                                                            on_input: move |event: Event<FormData>| {
                                                                dropdown_value.set(None);
                                                                deposit
                                                                    .deposit_mut()
                                                                    .with_mut(|w| w.dest = DepositTo::Address(event.value()));
                                                            },
                                                            on_keypress: move |_| {},
                                                            on_click: move |_| {},
                                                        }
                                                    },
                                                    DepositKreivoTabs::Community => rsx!{
                                                        Dropdown {
                                                            class: "payment__wallet dropdown--left".to_string(),
                                                            value: dropdown_value(),
                                                            label: t!("deposit-form-community-label"),
                                                            size: ElementSize::Medium,
                                                            placeholder: t!("header-cta-account"),
                                                            default: None,
                                                            on_change: move |event: usize| {
                                                                on_handle_community.send(event as u8);
                                                            },
                                                            body: community_items
                                                        }
                                                    },
                                                },
                                                Input {
                                                    message: deposit.get_deposit().amount,
                                                    placeholder: t!("deposit-form-amount-placeholder"),
                                                    label: t!("deposit-form-amount-label"),
                                                    error: None,
                                                    right_text: rsx! {
                                                        span { class: "input--right__text", "KSM" }
                                                    },
                                                    on_input: move |event: Event<FormData>| {
                                                        deposit
                                                            .deposit_mut()
                                                            .with_mut(|w| {
                                                                w.amount = event.value();
                                                            })
                                                    },
                                                    on_keypress: move |_| {},
                                                    on_click: move |_| {}
                                                }
                                            }
                                        }
                                        Button {
                                            text: t!("deposit-form-cta-continue"),
                                            disabled: !deposit.is_form_complete(),
                                            size: ElementSize::Medium,
                                            on_click: move |_| {
                                                spawn(
                                                    async move {
                                                        tooltip
                                                            .handle_tooltip(TooltipItem {
                                                                title: t!("deposit-tips-loading-title"),
                                                                body: t!("deposit-tips-loading-description"),
                                                                show: true,
                                                            });
                                                        let (destination, amount, to_community) = deposit
                                                            .get_deposit()
                                                            .to_deposit()
                                                            .map_err(|e| match e {
                                                                DepositError::MalformedAddress => {
                                                                    t!("errors-wallet-account_address")
                                                                }
                                                                DepositError::InvalidAmount => {
                                                                    t!("errors-form-invalid_amount")
                                                                }
                                                            })?;
                                                        let destination_js = convert_to_jsvalue(&destination)
                                                            .map_err(|_| {
                                                                warn!("Malformed dest account");
                                                                t!("errors-form-invalid_address")
                                                            })?;
                                                        depositAction(destination_js, amount, to_community)
                                                            .await
                                                            .map_err(|e| {
                                                                warn!("Deposit failed {:?}", e);
                                                                t!("errors-form-deposit_failed")
                                                            })?;
                                                        tooltip.hide();
                                                        info!("deposited {:?} to {:?}", amount, destination);
                                                        notification
                                                            .handle_success(
                                                                &t!("deposit-tips-created-description"),
                                                            );
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
                }
            }
        }
    )
}
