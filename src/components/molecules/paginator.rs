use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

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
    #[props(default = 1)]
    to: usize,
    on_change: EventHandler<PaginatorValue>,
}

pub fn Paginator(props: PaginatorProps) -> Element {
    let i18 = use_i18();
    let mut current_page = use_signal::<usize>(|| 1);

    rsx!(
        div { class: "paginator",
            span { class: "paginator__range",
                {translate!(i18, "dashboard.footer.paginator", from: current_page(), to: props.to)}
            }
            div { class: "paginator__jumpers",
                IconButton {
                    class: "button--avatar button--paginator",
                    disabled: current_page() <= 1,
                    size: ElementSize::Small,
                    body: rsx!(Icon { icon : ArrowLeft, height : 24, width : 24, fill : "var(--white)" }),
                    on_click: move |_| {
                        let current = current_page();
                        current_page.set(current - 1);
                        props
                            .on_change
                            .call(PaginatorValue {
                                value: current_page(),
                            });
                    }
                }
                IconButton {
                    class: "button--avatar button--paginator",
                    size: ElementSize::Small,
                    disabled: current_page() >= props.to,
                    body: rsx!(Icon { icon : ArrowRight, height : 24, width : 24, fill : "var(--white)" }),
                    on_click: move |_| {
                        let current = current_page();
                        current_page.set(current + 1);
                        props
                            .on_change
                            .call(PaginatorValue {
                                value: current_page(),
                            });
                    }
                }
            }
        }
    )
}
