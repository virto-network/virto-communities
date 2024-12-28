use dioxus_i18n::{prelude::*, t};
use std::str::FromStr;
use unic_langid::{langid, LanguageIdentifier};
use web_sys::window;
static EN_US: &str = include_str!("../locales/en-US.ftl");
static ES_ES: &str = include_str!("../locales/es-ES.ftl");
pub fn use_language() {
    let navigator_language = window()
        .expect("window")
        .navigator()
        .language()
        .unwrap_or("en-US".to_string());
    let default_language = if navigator_language.starts_with("es") {
        I18nConfig::new(langid!("es-ES"))
    } else {
        I18nConfig::new(langid!("en-US"))
    };
    let configuration_language = default_language
        .with_locale(Locale::new_static(langid!("en-US"), &EN_US))
        .with_locale(Locale::new_static(langid!("es-ES"), &ES_ES));

    use_init_i18n(|| configuration_language);
}
