use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
#[component]
pub fn InitiativeConfirmation() -> Element {
    let i18 = use_i18();
    rsx!(
        div { class: "form__inputs" }
    )
}
