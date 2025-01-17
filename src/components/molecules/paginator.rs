use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::atoms::{dropdown::ElementSize, ArrowLeft, ArrowRight, Icon, IconButton};

#[derive(PartialEq, Clone)]
pub struct PaginatorValue {
    value: usize,
}

impl PaginatorValue {
    pub fn value(&self) -> usize {
        self.value
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct PaginatorProps {
    #[props(default = 0)]
    from: usize,
    #[props(default = 0)]
    to: usize,
    #[props(default = 0)]
    value: usize,
    #[props(default = false)]
    is_item_dotted: bool,
    on_change: EventHandler<PaginatorValue>,
}

pub fn Paginator(props: PaginatorProps) -> Element {
    

    rsx!(
        div { class: "paginator",
            if !props.is_item_dotted {
                span { class: "paginator__range",
                {t!("dashboard-footer-paginator", from: props.value, to: props.to)}
            }
            }
            div { class: "paginator__jumpers",
                IconButton {
                    class: "button--avatar button--paginator",
                    disabled: props.value <= props.from,
                    size: ElementSize::Small,
                    body: rsx! {
                        Icon { icon: ArrowLeft, height: 24, width: 24, fill: "var(--white)" }
                    },
                    on_click: move |_| {
                        props
                            .on_change
                            .call(PaginatorValue {
                                value: props.value - 1
                            });
                    }
                }
                if props.is_item_dotted {
                    div { class: "paginator__dots",
                        for i in 0..=props.to {
                            button {
                                class: "paginator__dot",
                                class: if props.value == i { "paginator__dot--active" },
                                onclick: move |_| {
                                    props
                                        .on_change
                                        .call(PaginatorValue {
                                            value: i,
                                        });
                                }
                            }
                        }
                    }
                }
                IconButton {
                    class: "button--avatar button--paginator",
                    size: ElementSize::Small,
                    disabled: props.value >= props.to,
                    body: rsx! {
                        Icon { icon: ArrowRight, height: 24, width: 24, fill: "var(--white)" }
                    },
                    on_click: move |_| {
                        props
                            .on_change
                            .call(PaginatorValue {
                                value: props.value + 1
                            });
                    }
                }
            }
        }
    )
}
