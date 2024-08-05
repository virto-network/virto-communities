use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::atoms::{
        dropdown::ElementSize, input_tags::InputTagsEvent, markdown::MarkdownEvent, Input,
        InputTags, Markdown,
    },
    hooks::{use_initiative::use_initiative, use_onboard::use_onboard},
};

#[component]
pub fn InitiativeInfo(error: bool) -> Element {
    let i18 = use_i18();
    let onboard = use_onboard();
    let mut initiative = use_initiative();

    let mut name_maxlength = use_signal(|| 24);

    rsx!(
        div { class: "form__inputs form__inputs--initiative",
            div { class: "form__input form__input--initiative",
                div { class: "form__input__info",
                    span { class: "form__input__info__title",
                        {translate!(i18, "initiative.steps.info.name.label")}
                    }
                    p { class: "form__input__info__description",
                        {translate!(i18, "initiative.steps.info.name.description")}
                    }
                }
                Input {
                    message: initiative.get_info().name,
                    size: ElementSize::Small,
                    placeholder: translate!(i18, "initiative.steps.info.name.placeholder"),
                    error: if error {
                        if initiative.get_info().name.is_empty() {
                            Some(translate!(i18, "errors.form.not_empty"))
                        } else {
                            None
                        }
                    } else { None },
                    maxlength: 150,
                    required: true,
                    on_input: move |event: Event<FormData>| {
                        if event.value().len() < 150 {
                            initiative.info_mut().with_mut(|info| info.name = event.value());
                        } else {
                            name_maxlength.set(event.value().chars().count().try_into().expect("Should convert usize into u8") );
                        }
                    },
                    on_keypress: move |_| {},
                    on_click: move |_| {},
                }
            }
            hr { class: "form__divider" }
            div { class: "form__input form__input--initiative",
                div { class: "form__input__info",
                    span { class: "form__input__info__title",
                        {translate!(i18, "initiative.steps.info.description.label")}
                    }
                    p { class: "form__input__info__description",
                        {translate!(i18, "initiative.steps.info.description.description")}
                    }
                }
                Markdown {
                    content: initiative.get_info().description,
                    on_input: move |event: MarkdownEvent| {
                        initiative.info_mut().with_mut(|info| info.description = event.value);
                    },
                }
            }
            hr { class: "form__divider" }
            div { class: "form__input form__input--initiative",
                div { class: "form__input__info",
                    span { class: "form__input__info__title",
                        {translate!(i18, "initiative.steps.info.categories.label")}
                    }
                    p { class: "form__input__info__description",
                    {translate!(i18, "initiative.steps.info.categories.description")}
                    }
                }
                InputTags {
                    message: initiative.get_info().categories.join(","),
                    size: ElementSize::Small,
                    placeholder: translate!(i18, "initiative.steps.info.categories.placeholder"),
                    error: if error {
                        if onboard.get_basics().industry.is_empty() {
                            Some(translate!(i18, "errors.form.not_empty"))
                        } else {
                            None
                        }
                    } else { None },
                    maxlength: 5,
                    required: true,
                    on_input: move |event: InputTagsEvent| {
                        initiative.info_mut().with_mut(|info| info.categories = event.tags);
                    },
                    on_keypress: move |_| {},
                    on_click: move |_| {},
                }
            }
        }
    )
}
