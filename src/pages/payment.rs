use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use futures_util::TryFutureExt;
use gloo::utils::format::JsValueSerdeExt;
use pjs::PjsExtension;
use serde::Serialize;

use crate::{
    components::{
        atoms::{
            dropdown::{DropdownItem, ElementSize}, icon_button::Variant as IconButtonVariant, AccountButton, BankCardLine, Button, CheckboxCard, Dot, Dropdown, EditLine, Icon, IconButton, KusamaLogo, PaymentMethod, PaypalLogo, PolygonLogo, Title, VirtoLogo
        },
        molecules::header::set_signer,
    },
    hooks::{
        use_accounts::use_accounts,
        use_market_client::use_market_client,
        use_notification::{
            use_notification, NotificationHandle, NotificationHandler, NotificationItem,
            NotificationVariant,
        },
        use_onboard::use_onboard,
        use_our_navigator::use_our_navigator,
        use_session::{use_session, UserSession},
        use_spaces_client::use_spaces_client,
        use_tooltip::{use_tooltip, TooltipItem},
    },
    middlewares::is_dao_owner::is_dao_owner,
    services::{
        bot::types::CommunitySpace, kreivo::community_track::tracksIds, market::types::Tokens,
    },
};
use serde_json::{to_value, Error};
use wasm_bindgen::prelude::*;

const COMMUNITY_VALUE: f64 = 0.51;
const IDENTITY_VALUE: f64 = 0.11;
const MEMBERSHIP_VALUE: f64 = 0.3;

pub enum PaymentMethods {
    Card,
    Paypal,
    PSE,
    KUSAMA,
    DOT,
    ETH,
    None,
}

const APP_NAME: &str = "Virto";

#[derive(Serialize)]
struct Identity {
    pub display: String,
    // TODO: enable this to integrate the actual required fields by blockchain
    // pub legal: Option<String>,
    // pub web: Option<String>,
    pub matrix: Option<String>,
    // pub pgpFingerprint: Option<JsValue>,
    // pub image: Option<JsValue>,
    // pub twitter: Option<JsValue>,
    // pub github: Option<JsValue>,
    // pub discord: Option<JsValue>,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
enum DecisionMethod {
    Membership,
    _Rank,
    _NativeToken,
    _CommunityAsset { id: String, min_vote: i32 },
}

pub fn convert_to_jsvalue<T: Serialize>(value: &T) -> Result<JsValue, Error> {
    to_value(value)
        .map(|t: serde_json::Value| JsValue::from_serde(&t))
        .unwrap_or_else(|_| Ok(JsValue::from_str("Error creating JsValue")))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = window, js_name = topupThenCreateCommunity)]
    async fn topup_then_create_community(
        community_id: u16,
        name: String,
        decision_method: JsValue,
        maybe_identity: JsValue,
        maybe_memberships: JsValue,
        maybe_topup: JsValue,
    ) -> Result<JsValue, JsValue>;
}

#[component]
pub fn Payment() -> Element {
    let i18 = use_i18();
    let onboard = use_onboard();
    let market_client = use_market_client();

    let mut to_pay = consume_context::<Signal<f64>>();
    let mut ksm_market_price = use_signal::<f64>(|| 0.0);
    let mut payment_selected = use_signal(|| PaymentMethods::None);

    let mut accounts = use_accounts();
    let mut notification = use_notification();
    let mut tooltip = use_tooltip();
    let mut session = use_session();
    let nav = use_our_navigator();
    let spaces_client = use_spaces_client();
    let mut id_number = use_signal::<String>(|| String::new());

    let mut handle_required_inputs = use_signal::<bool>(|| false);
    let mut fee = use_signal::<f64>(|| 0.0);

    let get_account = move || {
        let Some(user_session) = session.get() else {
            return None;
        };

        accounts.get_one(user_session.account_id)
    };

    let members = onboard
        .get_invitations()
        .into_iter()
        .filter_map(|invitation| {
            if !invitation.account.is_empty() {
                Some(invitation.account)
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    let membership_value = members.len() as f64 * MEMBERSHIP_VALUE;
    let subtotal_ksm = COMMUNITY_VALUE + IDENTITY_VALUE + membership_value;

    to_pay.set(subtotal_ksm);

    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        let Ok(price) = market_client.get().get_price_by_token(Tokens::KSM).await else {
            log::error!("Price not found for {}", Tokens::KSM.name());
            return;
        };

        ksm_market_price.set(price);
    });

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
        let x = is_dao_owner(i18, accounts, notification)();
        log::info!("{x:?}");
        let account = &accounts.get()[event as usize];

        let Ok(serialized_session) = serde_json::to_string(&UserSession {
            name: account.name(),
            account_id: event,
            address: account.address(),
        }) else {
            return notification.handle_error(&translate!(i18, "errors.session.persist"));
        };

        session.persist_session_file(&serialized_session);
        session.update_account(event);

        accounts.set_account(Some(account.clone()));
        set_signer(account.address().clone());

        let account = get_account().and_then(|account| {
            Some(DropdownItem {
                key: account.address().clone(),
                value: account.name(),
            })
        });

        dropdown_value.set(account);
    };

    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        if session.is_logged() {
            match PjsExtension::connect(APP_NAME).await {
                Ok(mut vault) => {
                    let Ok(_) = vault.fetch_accounts().await else {
                        notification
                            .handle_error(&translate!(i18, "errors.wallet.accounts_not_found"));
                        return;
                    };

                    let vault_accounts = vault.accounts();
                    accounts.set(vault_accounts);

                    if let Some(user_session) = session.get() {
                        on_handle_account(user_session.account_id);
                    }
                }
                Err(pjs::Error::NoPermission) => {
                    session.persist_session_file("");
                }
                Err(_) => todo!(),
            }
        }
    });

    rsx!(
        div { class: "page page--payment",
            div { class: "payment__form",
                div { class: "form__wrapper",
                    Icon {
                        icon: VirtoLogo,
                        height: 64,
                        width: 64,
                        stroke_width: 1,
                        fill: "var(--color-lavanda-400)"
                    }
                    div { class: "form__title",
                        span { class: "label",
                            {translate!(i18, "onboard.payment.label")}
                        }
                        Title {
                            text: translate!(i18, "onboard.payment.title")
                        }
                    }
                    div { class: "row payment__row",
                        div { class: "summary",
                            h4 { class: "summary__subtitle", {translate!(i18, "onboard.payment.subtitle")} }
                            div { class: "payment__methods",
                                CheckboxCard {
                                    id: "a".to_string(),
                                    name: String::from("management"),
                                    checked: matches!(*payment_selected.read(), PaymentMethods::Card),
                                    soon: true,
                                    class: "checkbox-card--payment",
                                    body: rsx!(
                                        PaymentMethod {
                                            title: translate!(i18, "onboard.payment.methods.card.title"),
                                            fee: translate!(i18, "onboard.payment.methods.card.fee", fee: 5),
                                            icon: rsx!(
                                                Icon {
                                                    icon: BankCardLine,
                                                    height: 20,
                                                    width: 20,
                                                    fill: "var(--fill-600)"
                                                }
                                            ),
                                        }
                                    ),
                                    on_change: move |_| {
                                        payment_selected.set(PaymentMethods::Card);
                                        fee.set(0.05);
                                    },
                                }
                                CheckboxCard {
                                    id: "a".to_string(),
                                    name: String::from("management"),
                                    checked: matches!(*payment_selected.read(), PaymentMethods::Paypal),
                                    soon: true,
                                    class: "checkbox-card--payment",
                                    body: rsx!(
                                        PaymentMethod {
                                            title: translate!(i18, "onboard.payment.methods.paypal.title"),
                                            fee: translate!(i18, "onboard.payment.methods.paypal.fee", fee: 5),
                                            icon: rsx!(
                                                Icon {
                                                    icon: PaypalLogo,
                                                    height: 20,
                                                    width: 20,
                                                    fill: "var(--fill-600)"
                                                }
                                            ),
                                        }
                                    ),
                                    on_change: move |_| {
                                        payment_selected.set(PaymentMethods::Paypal);
                                        fee.set(0.05);
                                    },
                                }
                                CheckboxCard {
                                    id: "a".to_string(),
                                    name: String::from("management"),
                                    checked: matches!(*payment_selected.read(), PaymentMethods::PSE),
                                    soon: true,
                                    class: "checkbox-card--payment",
                                    body: rsx!(
                                        PaymentMethod {
                                            title: translate!(i18, "onboard.payment.methods.pse.title"),
                                            fee: translate!(i18, "onboard.payment.methods.pse.fee", fee: 3),
                                            icon: rsx!(
                                                Icon {
                                                    icon: PaypalLogo,
                                                    height: 20,
                                                    width: 20,
                                                    fill: "var(--fill-600)"
                                                }
                                            ),
                                        }
                                    ),
                                    on_change: move |_| {
                                        payment_selected.set(PaymentMethods::PSE);
                                        fee.set(0.03);
                                    },
                                }
                                CheckboxCard {
                                    id: "a".to_string(),
                                    name: String::from("management"),
                                    checked: matches!(*payment_selected.read(), PaymentMethods::KUSAMA),
                                    class: "checkbox-card--payment",
                                    body: rsx!(
                                        PaymentMethod {
                                            title: translate!(i18, "onboard.payment.methods.kusama.title"),
                                            fee: translate!(i18, "onboard.payment.methods.kusama.fee"),
                                            icon: rsx!(
                                                Icon {
                                                    icon: KusamaLogo,
                                                    height: 20,
                                                    width: 20,
                                                    fill: "var(--fill-600)"
                                                }
                                            ),
                                        }
                                    ),
                                    editable: rsx!(
                                        Dropdown {
                                            class: "payment__wallet dropdown--left".to_string(),
                                            value: dropdown_value(),
                                            size: ElementSize::Big,
                                            placeholder: translate!(i18, "header.cta.account"),
                                            default: None,
                                            on_change: move |event: usize| {
                                                on_handle_account(event as u8);
                                            },
                                            body: items
                                        }
                                    ),
                                    on_change: move |_| {
                                        payment_selected.set(PaymentMethods::KUSAMA);
                                        fee.set(0.0);
                                    },
                                }
                                CheckboxCard {
                                    id: "a".to_string(),
                                    name: String::from("management"),
                                    checked: matches!(*payment_selected.read(), PaymentMethods::DOT),
                                    class: "checkbox-card--payment",
                                    soon: true,
                                    body: rsx!(
                                        PaymentMethod {
                                            title: translate!(i18, "onboard.payment.methods.dot.title"),
                                            fee: translate!(i18, "onboard.payment.methods.dot.fee"),
                                            icon: rsx!(
                                                Icon {
                                                    icon: Dot,
                                                    height: 20,
                                                    width: 20,
                                                    fill: "var(--fill-600)"
                                                }
                                            ),
                                        }
                                    ),
                                    on_change: move |_| {
                                        payment_selected.set(PaymentMethods::DOT);
                                        fee.set(0.0);
                                    },
                                }
                                CheckboxCard {
                                    id: "a".to_string(),
                                    name: String::from("management"),
                                    checked: matches!(*payment_selected.read(), PaymentMethods::ETH),
                                    soon: true,
                                    class: "checkbox-card--payment",
                                    body: rsx!(
                                        PaymentMethod {
                                            title: translate!(i18, "onboard.payment.methods.eth.title"),
                                            fee: translate!(i18, "onboard.payment.methods.eth.fee"),
                                            icon: rsx!(
                                                Icon {
                                                    icon: PolygonLogo,
                                                    height: 20,
                                                    width: 20,
                                                    fill: "var(--fill-600)"
                                                }
                                            ),
                                        }
                                    ),
                                    on_change: move |_| {
                                        payment_selected.set(PaymentMethods::ETH);
                                        fee.set(0.0);
                                    },
                                }
                            }
                        }
                        div { class: "summary",
                            h4 { class: "summary__subtitle", {translate!(i18, "onboard.checkout.summary.subtitle")} }
                            article { class: "summary__details",
                                section { class: "summary__item",
                                    span { class: "summary__item__label", {translate!(i18, "onboard.checkout.summary.account.label")} }
                                    div { class: "summary__item__values",
                                        span { class: "summary__item__value", {format!("{:.2} KSM", COMMUNITY_VALUE)} }
                                        if ksm_market_price() > 0.0 {
                                            small { class: "summary__item__value", {format!("${:.2} USD", ksm_market_price() * COMMUNITY_VALUE)} }
                                        }
                                    }
                                }
                                section { class: "summary__item",
                                    span { class: "summary__item__label", {translate!(i18, "onboard.checkout.summary.verification.label")} }
                                    div { class: "summary__item__values",
                                        span { class: "summary__item__value", {format!("{:.2} KSM", IDENTITY_VALUE)} }
                                        if ksm_market_price() > 0.0 {
                                            small { class: "summary__item__value", {format!("${:.2} USD", ksm_market_price() * IDENTITY_VALUE)} }
                                        }
                                    }
                                }
                                section { class: "summary__item",
                                    span { class: "summary__item__label", {translate!(i18, "onboard.checkout.summary.membership.label")} }
                                    div { class: "summary__item__values",
                                        span { class: "summary__item__value summary__item__value--editable",
                                        {format!("x{} {}", members.len(), translate!(i18, "onboard.checkout.summary.membership.users"))}
                                            IconButton {
                                                variant: IconButtonVariant::Round,
                                                size: ElementSize::Big,
                                                class: "button--edit bg--transparent",
                                                body: rsx!(
                                                    Icon {
                                                        icon: EditLine,
                                                        height: 16,
                                                        width: 16,
                                                        fill: "var(--state-primary-active)"
                                                    }
                                                ),
                                                on_click: move |_| {
                                                    nav.push(vec![], "/onboarding")
                                                }
                                            }
                                        }
                                        if ksm_market_price() > 0.0 {
                                            small { class: "summary__item__value", {format!("${:.2} USD", ksm_market_price() * MEMBERSHIP_VALUE)} }
                                        }
                                    }
                                    div { class: "summary__item__values",
                                        span { class: "summary__item__value", {format!("{:.2} KSM", membership_value)} }
                                        if ksm_market_price() > 0.0 {
                                            small { class: "summary__item__value", {format!("${:.2} USD", ksm_market_price() * membership_value)} }
                                        }
                                    }
                                }
                            }

                            hr { class: "divider" }

                            section { class: "summary__item",
                                span { class: "summary__item__label--subtitle", {translate!(i18, "onboard.checkout.summary.subtotal.label")} }
                                div { class: "summary__item__values",
                                    span { class: "summary__item__value", {format!("{:.2} KSM", subtotal_ksm)} }
                                    if ksm_market_price() > 0.0 {
                                        small { class: "summary__item__value",
                                            {
                                                format!("${:.2} USD",
                                                    round_price(ksm_market_price() * COMMUNITY_VALUE)
                                                    + round_price(ksm_market_price() * IDENTITY_VALUE)
                                                    + round_price(ksm_market_price() * membership_value)
                                                )
                                            }
                                        }
                                    }
                                }
                            }

                            section { class: "summary__item",
                                span { class: "summary__item__label--subtitle", {translate!(i18, "onboard.checkout.summary.fee.label")} }
                                div { class: "summary__item__values",
                                    span { class: "summary__item__value", {
                                        if fee() == 0.0 {
                                            format!("{:.2} KSM", fee() * subtotal_ksm)
                                        } else {
                                            format!("({}%) {:.2} KSM", (fee() * 100.0) as u8, fee() * subtotal_ksm)
                                        }
                                    } }
                                    if ksm_market_price() > 0.0 {
                                        small { class: "summary__item__value",
                                            {
                                                format!("${:.2} USD",
                                                    round_price(ksm_market_price() * fee() * subtotal_ksm)
                                                )
                                            }
                                        }
                                    }
                                }
                            }

                            section { class: "summary__item",
                                span { class: "summary__item__label--subtitle", {translate!(i18, "onboard.checkout.summary.total.label")} }
                                div { class: "summary__item__values",
                                    span { class: "summary__item__value", {format!("{:.2} KSM", fee() * subtotal_ksm + subtotal_ksm)} }
                                    if ksm_market_price() > 0.0 {
                                        small { class: "summary__item__value",
                                            {
                                                format!("${:.2} USD",
                                                    round_price(ksm_market_price() * COMMUNITY_VALUE)
                                                    + round_price(ksm_market_price() * IDENTITY_VALUE)
                                                    + round_price(ksm_market_price() * membership_value)
                                                    + round_price(ksm_market_price() * fee() * subtotal_ksm)
                                                )
                                            }
                                        }
                                    }
                                }
                            }

                            Button {
                                class: "",
                                text: translate!(i18, "onboard.checkout.cta.next"),
                                size: ElementSize::Big,
                                disabled: matches!(*payment_selected.read(), PaymentMethods::None),
                                on_click: move |_| {
                                    if onboard.get_basics().name.is_empty() || onboard.get_basics().industry.is_empty() {
                                        handle_required_inputs.set(true);
                                        nav.push(vec!(), "/onboarding");
                                        return;
                                    } else {
                                        handle_required_inputs.set(false);
                                    }

                                    spawn({
                                        async move {
                                            let community = CommunitySpace {
                                                name: onboard.get_basics().name,
                                                logo: onboard.get_basics().logo,
                                                description: if onboard.get_basics().description.is_empty() {
                                                    None
                                                } else {
                                                    Some(onboard.get_basics().description)
                                                },
                                                industry: onboard.get_basics().industry,
                                            };
                                            let response_track_ids = tracksIds()
                                                .await
                                                .map_err(|_| {
                                                    translate!(i18, "errors.form.community_creation")
                                                })?;
                                            let name_bytes = Vec::from(onboard.get_basics().name);
                                            let community_id = name_bytes
                                                .into_iter()
                                                .fold(0u16, |acc, elem| acc + elem as u16);
                                            let mut offset = 0u16;
                                            while response_track_ids
                                                .communities
                                                .contains(&(community_id + offset))
                                            {
                                                offset += 1u16;
                                            }
                                            let current_id = community_id + offset;
                                            id_number.set(current_id.to_string());
                                            tooltip
                                                .handle_tooltip(TooltipItem {
                                                    title: translate!(i18, "onboard.tips.loading.title"),
                                                    body: translate!(i18, "onboard.tips.loading.description"),
                                                    show: true,
                                                });
                                            let decision_method = convert_to_jsvalue(
                                                    &DecisionMethod::Membership,
                                                )
                                                .map_err(|_| {
                                                    log::warn!("Malformed decision method");
                                                    translate!(i18, "errors.form.community_creation")
                                                })?;
                                            let response = spaces_client
                                                .get()
                                                .create(community)
                                                .await
                                                .map_err(|_| {
                                                    translate!(i18, "errors.form.community_creation")
                                                })?;
                                            let identity = Identity {
                                                display: onboard.get_basics().name,
                                                matrix: Some(response.get_id()),
                                            };
                                            let encoded_identity = convert_to_jsvalue(&identity)
                                                .map_err(|_| {
                                                    log::warn!("Malformed identity");
                                                    translate!(i18, "errors.form.community_creation")
                                                })?;
                                            let members = onboard
                                                .get_invitations()
                                                .into_iter()
                                                .filter_map(|invitation| {
                                                    if !invitation.account.is_empty() {
                                                        Some(invitation.account)
                                                    } else {
                                                        None
                                                    }
                                                })
                                                .collect::<Vec<String>>();
                                            let membership_accounts = convert_to_jsvalue(&members)
                                                .map_err(|_| {
                                                    log::warn!("Malformed membership accounts");
                                                    translate!(i18, "errors.form.community_creation")
                                                })?;
                                            topup_then_create_community(
                                                    current_id,
                                                    identity.display.clone(),
                                                    decision_method,
                                                    encoded_identity,
                                                    membership_accounts,
                                                    JsValue::UNDEFINED,
                                                )
                                                .await
                                                .map_err(|_| {
                                                    log::warn!("Error on xcm program");
                                                    translate!(i18, "errors.form.community_creation")
                                                })?;
                                            tooltip.hide();
                                            notification
                                                .handle_notification(NotificationItem {
                                                    title: translate!(i18, "onboard.tips.created.title"),
                                                    body: translate!(i18, "onboard.tips.created.description"),
                                                    variant: NotificationVariant::Success,
                                                    show: true,
                                                    handle: NotificationHandle {
                                                        value: NotificationHandler::None,
                                                    },
                                                });
                                            nav.push(Vec::new(), "/");
                                            Ok::<(), String>(())
                                        }
                                            .unwrap_or_else(move |e: String| {
                                                tooltip.hide();
                                                notification.handle_error(&e);
                                            })
                                    });
                                },
                                status: None,
                            }
                            p { class: "summary__rate", {format!("{} 1 KSM = {:.2} USD", translate!(i18, "onboard.checkout.summary.rate"), ksm_market_price())} }
                        }
                    }
                }
            }
        }
    )
}

fn round_price(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}
