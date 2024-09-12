use dioxus::prelude::*;

#[component]
pub fn CardSkeleton() -> Element {
    rsx!(
        section { class: "card card--skeleton",
            div { class: "card__container",
            }
        }
    )
}