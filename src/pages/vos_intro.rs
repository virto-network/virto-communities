use dioxus::{logger::tracing::debug, prelude::*};
use dioxus_i18n::t;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, ScrollToOptions};

use crate::{
    components::{
        atoms::{
            button::Variant, dropdown::ElementSize, ArrowRight, Button, CardVos, GlobalLine,
            HandCoinLine, Icon, TimerLine, WalletLine,
        },
        molecules::{paginator::PaginatorValue, Paginator},
    },
    hooks::use_our_navigator::use_our_navigator,
};

pub struct IntroCards {
    title: String,
    description: String,
    icon_element: Element,
}

#[component]
pub fn VOSIntro() -> Element {
    
    let nav = use_our_navigator();
    let mut show_cards = use_signal(|| false);

    let mut swiper_ref = use_signal::<Option<Box<HtmlElement>>>(|| None);
    let mut current_card = use_signal(|| 0);

    let intro_cards: Vec<IntroCards> = vec![
        IntroCards {
            title: t!("vos_intro-cards-management-title"),
            description: t!("vos_intro-cards-management-description"),
            icon_element: rsx!(Icon {
                icon: TimerLine,
                height: 56,
                width: 56,
                fill: "var(--state-destructive-active)"
            }),
        },
        IntroCards {
            title: t!("vos_intro-cards-payments-title"),
            description: t!("vos_intro-cards-payments-description"),
            icon_element: rsx!(Icon {
                icon: GlobalLine,
                height: 56,
                width: 56,
                fill: "var(--state-destructive-active)"
            }),
        },
        IntroCards {
            title: t!("vos_intro-cards-governance-title"),
            description: t!("vos_intro-cards-governance-description"),
            icon_element: rsx!(Icon {
                icon: HandCoinLine,
                height: 56,
                width: 56,
                fill: "var(--state-destructive-active)"
            }),
        },
        IntroCards {
            title: t!("vos_intro-cards-wallet-title"),
            description: t!("vos_intro-cards-wallet-description"),
            icon_element: rsx!(Icon {
                icon: WalletLine,
                height: 56,
                width: 56,
                fill: "var(--state-destructive-active)"
            }),
        },
    ];

    let on_handle_paginator = move || {
        let Some(swiper_ref) = swiper_ref() else {
            return;
        };
        let Some(first_child) = swiper_ref.first_element_child() else {
            return;
        };
        let card_width = first_child.get_bounding_client_rect().width() + 24.0;
        let scroll_left_calc: f64 = card_width * current_card() as f64;
        let scroll_options = ScrollToOptions::default();
        scroll_options.set_behavior(web_sys::ScrollBehavior::Smooth);
        scroll_options.set_left(scroll_left_calc);
        swiper_ref.scroll_to_with_scroll_to_options(&scroll_options);
    };

    rsx! {
        div {
            class: "vos-intro grid-main",
            class: if show_cards() { "vos-intro--tour" },
            div {
                class: "vos-intro__head",
                div { class: "vos-intro__name",
                    span {
                       {t!("vos_intro-name")}
                    }
                }
                p { class: "vos-intro__description",
                    {t!("vos_intro-description_p1")}
                    span { class: "vos-intro__description vos-intro__description--green",
                        {t!("vos_intro-description_p2")}
                    }
                }
            }
            if show_cards() {
                div { class: "vos-intro__cards",
                    onmounted: move |event| {
                        event
                            .data
                            .downcast::<web_sys::Element>()
                            .and_then(|element| element.clone().dyn_into::<web_sys::HtmlElement>().ok())
                            .map(|html_element| swiper_ref.set(Some(Box::new(html_element.clone()))));
                    },
                    for (index, card) in intro_cards.iter().enumerate() {
                        CardVos {
                            title: card.title.clone(),
                            description: card.description.clone(),
                            active: current_card() == index,
                            icon: card.icon_element.clone(),
                            on_click: move |_| {
                                current_card.set(index);
                                on_handle_paginator();
                            },
                        }
                    }
                }
            }
            div { class: "vos-intro__ctas",
                if current_card() == intro_cards.len().saturating_sub(1).max(0) {
                    Button {
                        class: "vos-intro__next",
                        text: t!("vos_intro-continue"),
                        size: ElementSize::Medium,
                        status: None,
                        on_click: move |_| {
                            nav.push(vec![], "/");
                        },
                    }
                } else {
                    Button {
                        class: "vos-intro__skip",
                        text: t!("vos_intro-skip"),
                        size: ElementSize::Medium,
                        variant: Variant::Tertiary,
                        status: None,
                        on_click: move |_| {
                            nav.push(vec![], "/");
                        },
                    }
                }
                if show_cards() {
                    Paginator {
                        to: intro_cards.len().saturating_sub(1).max(0),
                        is_item_dotted: true,
                        value: current_card(),
                        on_change: move |event: PaginatorValue| {
                            debug!("{:?}", event.value());
                            current_card.set(event.value());
                            on_handle_paginator();
                        }
                    }
                } else {
                    Button {
                        class: "vos-intro__next",
                        text: t!("vos_intro-tour"),
                        size: ElementSize::Medium,
                        right_icon: rsx! {
                            Icon { icon: ArrowRight, height: 20, width: 20, fill: "var(--text-primary)" }
                        },
                        status: None,
                        on_click: move |_| {
                            show_cards.set(true);
                        },
                    }
                }
            }
        }
    }
}
