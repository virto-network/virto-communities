use std::str::FromStr;
use dioxus_std::i18n::{use_init_i18n, Language};
use unic_langid::LanguageIdentifier;
use web_sys::window;
static EN_US: &str = include_str!("../locales/en-US.json");
static ES_ES: &str = include_str!("../locales/es-ES.json");
pub fn use_language() {
    let navigator_language = window()
        .expect("window")
        .navigator()
        .language()
        .unwrap_or("en-US".to_string());
    let default_language = if navigator_language.starts_with("es") {
        "es-ES"
    } else {
        "en-US"
    };
    let selected_language: LanguageIdentifier = default_language
        .parse()
        .expect("can't parse es-ES language");
    let fallback_language: LanguageIdentifier = selected_language.clone();
    use_init_i18n(
        selected_language,
        fallback_language,
        || {
            let en_us = Language::from_str(EN_US).expect("can't get EN_US language");
            let es_es = Language::from_str(ES_ES).expect("can't get ES_ES language");
            vec![en_us, es_es]
        },
    );
}
