use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;

use crate::{
    components::atoms::{
        dropdown::ElementSize, icon_button::Variant, AddPlus, Close, Icon, IconButton,
    },
    hooks::{
        use_communities::use_communities, use_our_navigator::use_our_navigator, use_tabs::{use_tabs, Tab}
    }, services::kreivo::communities,
};
#[component]
pub fn Tab() -> Element {
    let i18 = use_i18();
    let mut tabs = use_tabs();
    let mut nav = use_our_navigator();
    let mut communities = use_communities();

    rsx!(
        div { class: "dashboard__tabs",
            for (index, tab) in tabs.get().iter().enumerate() {
                {
                    let path = tab.path.clone();

                    rsx!{
                        button {
                            class: "button button--tertiary tab-window",
                            class: if let Some(i) = tabs.selected() { if i == index { "tab-window--selected" } else { "" }} else { ""},
                            onclick: move |_| {
                                tabs.select(Some(index));
                                nav.push(vec![], &path);
                            },
                            h2 { class: "tab-window__name", {tab.name.clone()} }
                            IconButton {
                                variant: Variant::Ghost,
                                size: ElementSize::Small,
                                class: "button--avatar",
                                body: rsx!(Icon { icon : Close, height : 20, width : 20, fill : "var(--state-primary-active)" }),
                                on_click: move |_| {
                                    let latest_tab = tabs.remove(index);
                                    if let Some(t) = latest_tab {
                                        nav.push(vec![], &t.path);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "tab-window tab-window--new",
                IconButton {
                    variant: Variant::Round,
                    size: ElementSize::Small,
                    class: "button--avatar bg--state-primary-active",
                    body: rsx!(Icon { icon : AddPlus, height : 20, width : 20, fill : "var(--fill-00)" }),
                    on_click: move |_| {
                        let community = communities.get_community();
                        let path = format!("/dao/{}/plugins", community.id);
                        tabs.push(Tab {name: "Home".to_string(), path: path.clone()});
                        nav.push(vec![], &path);
                    }
                }
            }
        }
    )
}
