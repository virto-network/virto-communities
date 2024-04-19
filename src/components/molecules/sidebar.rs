use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::atoms::{avatar::Variant, AddPlus, Avatar, Icon, IconButton},
    pages::route::Route,
};

#[component]
pub fn Sidebar() -> Element {
    let i18 = use_i18();
    let nav = use_navigator();
    rsx!(
       section {
           class: "sidebar",
           div { class: "sidebar__container",
                ul { class: "sidebar__list",
                    IconButton {
                        body: rsx!(
                            Avatar {
                                name: "V",
                                size: 60,
                                uri: None,
                                variant: Variant::SemiRound
                            }
                        ),
                        on_click: move |_| { }
                    }
                    IconButton {
                        class: "button--icon",
                        body: rsx!(
                            Icon {
                                icon: AddPlus,
                                height: 60,
                                width: 60,
                                stroke_width: 1,
                                stroke: "var(--text-white)"
                            }
                        ),
                        on_click: move |_| {
                            nav.push(Route::Discover {});
                        }
                        span {
                            {translate!(i18, "sidebar.cta")}
                        }
                    }
                }
            }
       }
    )
}
