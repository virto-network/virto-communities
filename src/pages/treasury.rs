use crate::{
    components::atoms::{
        dropdown::{DropdownItem, ElementSize},
        AddLine, AddPlus, ArrowRightUpLine, Button, ChevronLeft, ChevronRight, ContactsBookLine,
        CurrencyLine, Dropdown, FileListLine, HandCoinLine, Icon, IconButton, ListBullet,
        More2Fill, Tab, WalletLine,
    },
    hooks::{
        use_accounts::use_accounts, use_communities::use_communities,
        use_notification::use_notification, use_our_navigator::use_our_navigator,
        use_tabs::use_tabs, use_timestamp::use_timestamp, use_tooltip::use_tooltip,
    },
};
use dioxus::prelude::*;
use dioxus_charts::LineChart;
use dioxus_std::{i18n::use_i18, translate};

#[derive(PartialEq)]
enum TreasuryTabs {
    Overview,
    Transactions,
    Recipients,
    Pay,
    Invoicing,
}

#[derive(PartialEq)]
enum AssetTabs {
    Account,
    Currency,
}

#[derive(PartialEq)]
enum MovementStatus {
    Completed,
    Pending,
    Rejected,
}

#[component]
pub fn Treasury(id: u16) -> Element {
    let i18 = use_i18();
    let mut tooltip = use_tooltip();
    let mut communities = use_communities();
    let mut tabs = use_tabs();
    let nav = use_our_navigator();
    let notification = use_notification();
    let accounts = use_accounts();
    let timestamp = use_timestamp();

    use_effect(use_reactive(
        (&communities.get_communities().len(),),
        move |(len,)| {
            if len > 0 {
                if let Err(_) = communities.set_community(id) {
                    let path = format!("/");
                    nav.push(vec![], &path);
                };
            }
        },
    ));

    let mut profile_value = use_signal(|| TreasuryTabs::Overview);
    let mut asset_tab_value = use_signal(|| AssetTabs::Account);
    let mut dropdown_value = use_signal::<Option<DropdownItem>>(|| None);
    let mut items = vec![];
    for account in accounts.get().into_iter() {
        items.push(rsx!(
            div { onclick: move |_| {}, "Hola" }
        ))
    }

    rsx! {
        div { class: "treasury grid-main",
            div { class: "card-treasury__head",
                div { class: "card-treasury__left",
                    h5 { class: "dashboard__head__subtitle", {communities.get_community().name}}
                    h3 { class: "dashboard__head__title", "Treasury"}
                }
                div { class: "head__actions",
                    Dropdown {
                        class: "header__wallet".to_string(),
                        value: dropdown_value(),
                        placeholder: "Move Money",
                        size: ElementSize::Medium,
                        default: None,
                        on_change: move |event: usize| {

                        },
                        body: items
                    }
                }
            }
            div { class: "dashboard__head",
                div { class: "account__options",
                    Tab {
                        text: "Overview",
                        is_active: if *profile_value.read() == TreasuryTabs::Overview { true } else { false },
                        on_click: move |_| {
                            profile_value.set(TreasuryTabs::Overview);
                        }
                    }
                    Tab {
                        text: "Transactions",
                        is_active: if *profile_value.read() == TreasuryTabs::Transactions { true } else { false },
                        right_icon: rsx!(
                            Icon { icon : ListBullet, height : 20, width : 20, stroke_width : 1, fill :
                            "var(--text-primary)" }
                        ),
                        on_click: move |_| {
                            profile_value.set(TreasuryTabs::Transactions);
                        }
                    }
                    Tab {
                        text: "Recipients",
                        is_active: if *profile_value.read() == TreasuryTabs::Recipients { true } else { false },
                        right_icon: rsx!(
                            Icon { icon : ContactsBookLine, height : 20, width : 20, stroke_width : 1, fill :
                            "var(--text-primary)" }
                        ),
                        on_click: move |_| {
                            profile_value.set(TreasuryTabs::Recipients);
                        }
                    }
                    Tab {
                        text: "Bill Pay",
                        is_active: if *profile_value.read() == TreasuryTabs::Pay { true } else { false },
                        right_icon: rsx!(
                            Icon { icon : CurrencyLine, height : 20, width : 20, stroke_width : 1, fill :
                            "var(--text-primary)" }
                        ),
                        on_click: move |_| {
                            profile_value.set(TreasuryTabs::Pay);
                        }
                    }
                    Tab {
                        text: "Invoicing",
                        is_active: if *profile_value.read() == TreasuryTabs::Invoicing { true } else { false },
                        right_icon: rsx!(
                            Icon { icon : FileListLine, height : 20, width : 20, stroke_width : 1, fill :
                            "var(--text-primary)" }
                        ),
                        on_click: move |_| {
                            profile_value.set(TreasuryTabs::Invoicing);
                        }
                    }
                }
            }
            match &*profile_value.read() {
                TreasuryTabs::Invoicing=>rsx!(
                    div{ class: "invoicing__wrapper",
                        div { class: "invoicing__header",
                            h3 { class: "invoicing__header__title",
                                "Invoicing"
                            }
                            Button {
                                text: "Create Invoice",
                                size: ElementSize::Small,
                                status: None,
                                right_icon: rsx! {
                                    Icon { icon: AddLine, height: 20, width: 20, fill: "var(--base-surface-1)" }
                                },
                                on_click: move |_| {
                                    nav.push(vec![], &format!("/dao/{id}/invoice"));
                                }
                            }
                        }
                        div { class: "invoicing__cards",
                            div { class: "invoicing__card",
                                span { class: "invoicing__card__left",
                                    {format!("${}", "2K")}
                                }
                                div { class: "invoicing__card__right",
                                    span { class: "invoicing__card__title",
                                        "Total Open"
                                    }
                                    span { class: "invoicing__card__info",
                                        "$18,149.18"
                                    }
                                }
                            }
                            div { class: "invoicing__card",
                                span { class: "invoicing__card__left invoicing__card__left--danger",
                                    {format!("${}", "615")}
                                }
                                div { class: "invoicing__card__right",
                                    span { class: "invoicing__card__title",
                                        "Overdue Invoices"
                                    }
                                    span { class: "invoicing__card__info",
                                        {format!("{} invoices", "2")}
                                    }
                                }
                            }
                            div { class: "invoicing__card",
                                span { class: "invoicing__card__left",
                                    {format!("${}", "840")}
                                }
                                div { class: "invoicing__card__right",
                                    span { class: "invoicing__card__title",
                                        "Paid Invoices"
                                    }
                                    span { class: "invoicing__card__info",
                                        {format!("{} invoices", "6")}
                                    }
                                }
                            }
                        }
                    }
                ),
                TreasuryTabs::Overview => rsx!(
                    div { class: "treasury__wrapper",
                        article { class: "row row--treasury",
                            section { class: "card-treasury",
                                div { class: "card-treasury__head",
                                    h3 { class: "card-treasury__title",
                                        "Balance"
                                    }
                                }
                                LineChart {
                                    width: "100%",
                                    height: "280px",
                                    padding_top: 30,
                                    padding_left: 50,
                                    padding_right: 90,
                                    padding_bottom: 30,
                                    show_grid_ticks: true,
                                    show_dotted_grid: false,
                                    label_interpolation: (|v| format!("{v}")) as fn(f32) -> String,
                                    series: vec![
                                        vec![75.77, 73.95, 74.56, 78.25, 77.15, 62.64, 67.51],
                                    ],
                                    labels: vec!["2016".into(), "2017".into(), "2018".into(), "2019".into(), "2020".into(), "2021".into(), "2022".into()],
                                    series_labels: vec![],
                                }
                            }
                            section { class: "card-treasury",
                                div { class: "card-treasury__head",
                                    div { class: "card-treasury__left",
                                        h3 { class: "card-treasury__title",
                                            "Assets"
                                        }
                                        div { class: "account__options",
                                            Tab {
                                                text: "by Account/Wallet",
                                                is_active: matches!(*asset_tab_value.read(), AssetTabs::Account),
                                                // is_active: if *asset_tab_value.read() == AssetTabs::Account { true } else { false },
                                                on_click: move |_| {
                                                    asset_tab_value.set(AssetTabs::Account);
                                                }
                                            }
                                            Tab {
                                                text: "by Currency",
                                                is_active: matches!(*asset_tab_value.read(), AssetTabs::Currency),
                                                // is_active: if *asset_tab_value.read() == AssetTabs::Currency { true } else { false },
                                                on_click: move |_| {
                                                    asset_tab_value.set(AssetTabs::Currency);
                                                }
                                            }
                                        }
                                    }
                                    div { class: "card-treasury__cta",
                                        IconButton {
                                            class: "button--avatar",
                                            size: ElementSize::Small,
                                            body: rsx!(
                                                Icon { icon : AddPlus, height : 20, width : 20, stroke_width : 2, fill :
                                                "var(--fill-00)" }
                                            ),
                                            on_click: move |_| {

                                            }
                                        }
                                        IconButton {
                                            class: "button--avatar bg--transparent",
                                            size: ElementSize::Small,
                                            body: rsx!(
                                                Icon { icon : More2Fill, height : 24, width : 24, stroke_width : 2, fill :
                                                "var(--state-primary-active)" }
                                            ),
                                            on_click: move |_| {

                                            }
                                        }
                                    }
                                }
                                match *asset_tab_value.read() {
                                    AssetTabs::Account => rsx!(
                                        ul { class: "card-treasury__list",
                                            li { class: "card-treasury__item",
                                                Icon { icon : WalletLine, height : 24, width : 24, fill : "var(--state-primary-active)" }
                                                span { class: "treasury__item",
                                                    span { class: "treasury__name", "Checking" }
                                                    span { class: "treasury__address", "*1469" }
                                                }
                                                span { class: "treasury__value", "$1,374" }
                                            }
                                            li { class: "card-treasury__item",
                                                Icon { icon : HandCoinLine, height : 24, width : 24, fill : "var(--state-primary-active)" }
                                                span { class: "treasury__item",
                                                    span { class: "treasury__name", "Savings" }
                                                    span { class: "treasury__address", "*1469" }
                                                }
                                                span { class: "treasury__value", "$1,374" }
                                            }
                                            li { class: "card-treasury__item",
                                                Icon { icon : ArrowRightUpLine, height : 24, width : 24, fill : "var(--state-primary-active)" }
                                                span { class: "treasury__item",
                                                    span { class: "treasury__name", "Growth DAO" }
                                                    span { class: "treasury__address", "*1469" }
                                                }
                                                span { class: "treasury__value", "$1,374" }
                                            }
                                            li { class: "card-treasury__item",
                                                Icon { icon : WalletLine, height : 24, width : 24, fill : "var(--state-primary-active)" }
                                                span { class: "treasury__item",
                                                    span { class: "treasury__name", "BTC Wallet" }
                                                    span { class: "treasury__address", "*1469" }
                                                }
                                                span { class: "treasury__value", "$1,374" }
                                            }
                                            li { class: "card-treasury__item",
                                                Icon { icon : WalletLine, height : 24, width : 24, fill : "var(--state-primary-active)" }
                                                span { class: "treasury__item",
                                                    span { class: "treasury__name", "ETH Wallet" }
                                                    span { class: "treasury__address", "*1469" }
                                                }
                                                span { class: "treasury__value", "$1,374" }
                                            }
                                            li { class: "card-treasury__item",
                                                Icon { icon : WalletLine, height : 24, width : 24, fill : "var(--state-primary-active)" }
                                                span { class: "treasury__item",
                                                    span { class: "treasury__name", "KSM Wallet" }
                                                    span { class: "treasury__address", "*1469" }
                                                }
                                                span { class: "treasury__value", "$1,374" }
                                            }
                                        }
                                    ),
                                    AssetTabs::Currency => rsx!(
                                        ul { class: "card-treasury__list",
                                            li { class: "card-treasury__item",
                                                img {
                                                    src: "/images/kusama_logo.png"
                                                }
                                                span { class: "treasury__name", "Kusama" }
                                                span { class: "treasury__item treasury__item--column",
                                                    span { class: "treasury__value", "$1,374 KSM" }
                                                    span { class: "treasury__address", "∼$34,374.58" }
                                                }
                                            }
                                        }
                                    ),
                                }
                            }
                        }
                        section { class: "movements__head",
                            h3 { class: "movements__title",
                                "Money Movement"
                            }
                            div { class: "date-pager",
                                IconButton {
                                    class: "button--avatar bg--transparent",
                                    size: ElementSize::Small,
                                    body: rsx!(
                                        Icon { icon : ChevronLeft, height : 20, width : 20, stroke_width : 2, fill :
                                        "var(--state-primary-active)" }
                                    ),
                                    on_click: move |_| {

                                    }
                                }
                                span { class: "date-pager__value",
                                    "Sep 2024"
                                }
                                IconButton {
                                    class: "button--avatar bg--transparent",
                                    size: ElementSize::Small,
                                    body: rsx!(
                                        Icon { icon : ChevronRight, height : 20, width : 20, stroke_width : 2, fill :
                                        "var(--state-primary-active)" }
                                    ),
                                    on_click: move |_| {

                                    }
                                }
                            }
                        }
                        article { class: "row row--treasury",
                            section { class: "card-movements",
                                div { class: "card-movements__head",
                                    div { class: "card-treasury__left",
                                            h6 { class: "card-movements__title",
                                                "Money In"
                                            }
                                            span { class: "card-treasury__amount card-treasury__amount--green",
                                                "$2,721"
                                            }
                                        }
                                    }
                                    hr { class: "divider divider--space-3" }
                                    ul { class: "card-movement__list",
                                        li { class: "card-movement__item",
                                            div { class: "movement__left",
                                                span { class: "movement__value", "$1,374" }
                                                span { class: "movement__item",
                                                    span { class: "movement__medium", "CRIPTO *1FG4" }
                                                    span { class: "movement__divider", "•" }
                                                    span { class: "movement__asset", "KSM" }
                                                }
                                            }
                                            span { class: "movement__status movement__status--completed", "Completed" }
                                        }
                                        li { class: "card-movement__item",
                                            div { class: "movement__left",
                                                span { class: "movement__value", "$1,374" }
                                                span { class: "movement__item",
                                                    span { class: "movement__medium", "CRIPTO *1FG4" }
                                                    span { class: "movement__divider", "•" }
                                                    span { class: "movement__asset", "KSM" }
                                                }
                                            }
                                            span { class: "movement__status movement__status--completed", "Completed" }
                                        }
                                        li { class: "card-movement__item",
                                            div { class: "movement__left",
                                                span { class: "movement__value", "$1,374" }
                                                span { class: "movement__item",
                                                    span { class: "movement__medium", "CRIPTO *1FG4" }
                                                    span { class: "movement__divider", "•" }
                                                    span { class: "movement__asset", "KSM" }
                                                }
                                            }
                                            span { class: "movement__status movement__status--pending", "Pending" }
                                        }
                                        li { class: "card-movement__item",
                                            div { class: "movement__left",
                                                span { class: "movement__value", "$1,374" }
                                                span { class: "movement__item",
                                                    span { class: "movement__medium", "CRIPTO *1FG4" }
                                                    span { class: "movement__divider", "•" }
                                                    span { class: "movement__asset", "KSM" }
                                                }
                                            }
                                            span { class: "movement__status movement__status--rejected", "Rejected" }
                                        }
                                    }
                                    hr { class: "divider divider--space-3" }
                                    div { class: "card-movements__head",
                                        div { class: "card-treasury__left",
                                            h6 { class: "card-movements__title",
                                                "Last 3 Months Average"
                                            }
                                        }
                                        div { class: "card-treasury__cta",
                                            span { class: "treasury__value",
                                                "$2,721"
                                            }
                                        }
                                    }
                                }
                                section { class: "card-movements",
                                    div { class: "card-movements__head",
                                    div { class: "card-treasury__left",
                                            h6 { class: "card-movements__title",
                                                "Money In"
                                            }
                                            span { class: "card-treasury__amount",
                                                "-$2,721"
                                            }
                                        }
                                    }
                                    hr { class: "divider divider--space-3" }
                                    ul { class: "card-movement__list",
                                        li { class: "card-movement__item",
                                            div { class: "movement__left",
                                                span { class: "movement__value", "-$1,374" }
                                                span { class: "movement__item",
                                                    span { class: "movement__medium", "CRIPTO *1FG4" }
                                                    span { class: "movement__divider", "•" }
                                                    span { class: "movement__asset", "KSM" }
                                                }
                                            }
                                            span { class: "movement__status movement__status--completed", "Completed" }
                                        }
                                    }
                                    hr { class: "divider divider--space-3" }
                                    div { class: "card-movements__head",
                                        div { class: "card-treasury__left",
                                            h6 { class: "card-movements__title",
                                                "Last 3 Months Average"
                                            }
                                        }
                                        div { class: "card-treasury__cta",
                                            span { class: "treasury__value",
                                                "-$2,721"
                                            }
                                        }
                                    }
                                }
                        }
                    }
                ),
                TreasuryTabs::Transactions => todo!(),
                TreasuryTabs::Recipients => todo!(),
                TreasuryTabs::Pay => rsx!(
                    div{ class: "bill-pay__wrapper",
                        div { class: "bill-pay__header",
                            h3 { class: "bill-pay__header__title",
                                "Bill pay"
                            }
                            Button {
                                text: "Add Bill",
                                size: ElementSize::Small,
                                status: None,
                                right_icon: rsx! {
                                    Icon { icon: AddLine, height: 20, width: 20, fill: "var(--base-surface-1)" }
                                },
                                on_click: move |_| {
                                    nav.push(vec![], &format!("/dao/{id}/bill"));
                                }
                            }
                        }
                        div { class: "bill-pay__cards",
                            div { class: "bill-pay__card",
                                span { class: "bill-pay__card__left",
                                    {format!("{}", "6")}
                                }
                                div { class: "bill-pay__card__right",
                                    span { class: "bill-pay__card__title",
                                        "Total Outstanding"
                                    }
                                    span { class: "bill-pay__card__info",
                                        "$18,149.18"
                                    }
                                }
                            }
                            div { class: "bill-pay__card",
                                span { class: "bill-pay__card__left bill-pay__card__left--danger",
                                    {format!("{}", "0")}
                                }
                                div { class: "bill-pay__card__right",
                                    span { class: "bill-pay__card__title",
                                        "Overdue"
                                    }
                                    span { class: "bill-pay__card__info",
                                        {format!("{}", "$18,149.18")}
                                    }
                                }
                            }
                            div { class: "bill-pay__card",
                                span { class: "bill-pay__card__left",
                                    {format!("{}", "0")}
                                }
                                div { class: "bill-pay__card__right",
                                    span { class: "bill-pay__card__title",
                                        "Due in next 7 days"
                                    }
                                    span { class: "bill-pay__card__info",
                                        {format!("{}", "$0.00")}
                                    }
                                }
                            }
                        }
                    }
                ),
            }
        }
    }
}
