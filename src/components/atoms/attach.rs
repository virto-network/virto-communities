use std::ops::Deref;

use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use futures_util::TryFutureExt;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::{
    components::atoms::{
        button::Variant as ButtonVariant, dropdown::ElementSize,
        icon_button::Variant as IconButtonVariant, Button, Close, Icon, IconButton,
    },
    hooks::use_attach::{use_attach, AttachFile},
};

#[derive(Clone, Debug)]
pub enum AttachError {
    NotFound,
    UncoverType,
    UnknownContent,
    Size,
}

#[derive(Clone, Debug)]
pub struct FeedAttachError {
    explanation: String,
    details: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct AttachEvent {
    pub value: Vec<u8>,
}

#[derive(PartialEq, Props, Clone)]
pub struct AttachProps {
    // value: Vec<u8>,
    label: Option<String>,
    supported_types: Vec<String>,
    cta_text: String,
    on_change: EventHandler<AttachFile>,
}

const MAX_FILE_SIZE: u64 = 2 * 1024 * 1024;

pub fn Attach(props: AttachProps) -> Element {
    let i18 = use_i18();
    let mut attach = use_attach();
    let mut textarea_ref = use_signal::<Option<Box<HtmlElement>>>(|| None);
    let mut error = use_signal::<Option<FeedAttachError>>(|| None);

    let supported_types = props
        .supported_types
        .iter()
        .map(|t| t.parse::<mime::Mime>().expect("Supported mime"))
        .collect::<Vec<mime::Mime>>();

    let on_handle_attach = move |_| {
        if let Some(input_element) = textarea_ref() {
            input_element.click();
        }
    };

    let on_handle_input = move |event: Event<FormData>| {
        let supported_types = supported_types.clone();
        spawn({
            async move {
                let files = &event.files().ok_or(AttachError::NotFound)?;
                let fs = files.files();

                let existing_file = fs.get(0).ok_or(AttachError::NotFound)?;
                let name = existing_file.clone();

                let content = files
                    .read_file(existing_file)
                    .await
                    .ok_or(AttachError::NotFound)?;
                let infered_type = infer::get(content.deref()).ok_or(AttachError::UncoverType)?;

                let content_type: Result<mime::Mime, _> = infered_type.mime_type().parse();
                let content_type = content_type.map_err(|_| AttachError::UnknownContent)?;

                if !supported_types.contains(&content_type) {
                    return Err(AttachError::UncoverType);
                }

                let blob = match content_type.type_() {
                    mime::IMAGE => gloo::file::Blob::new(content.deref()),
                    mime::VIDEO => gloo::file::Blob::new_with_options(
                        content.deref(),
                        Some(infered_type.mime_type()),
                    ),
                    _ => gloo::file::Blob::new(content.deref()),
                };

                let size = blob.size().clone();

                if size > MAX_FILE_SIZE {
                    return Err(AttachError::Size);
                }

                let object_url = gloo::file::ObjectUrl::from(blob);

                let attach_file = AttachFile {
                    name: existing_file.to_string(),
                    preview_url: object_url,
                    data: content.clone(),
                    content_type,
                    size,
                };

                attach.set(Some(attach_file.clone()));

                props.on_change.call(attach_file);

                Ok::<(), AttachError>(())
            }
            .unwrap_or_else(move |e: AttachError| {
                let message_error = match e {
                    AttachError::NotFound => FeedAttachError {
                        explanation: translate!(i18, "errors.attach.not_found.explanation"),
                        details: translate!(i18, "errors.attach.not_found.details"),
                    },
                    AttachError::Size => FeedAttachError {
                        explanation: translate!(i18, "errors.attach.size.explanation"),
                        details: translate!(i18, "errors.attach.size.details"),
                    },
                    AttachError::UncoverType | AttachError::UnknownContent => FeedAttachError {
                        explanation: translate!(i18, "errors.attach.mime.explanation"),
                        details: translate!(i18, "errors.attach.mime.details"),
                    },
                };

                error.set(Some(message_error))
            })
        });
    };

    rsx!(
        section {
            class: "attach",
            if let Some(value) = props.label {
                label { class: "input__label", "{value}" }
            }
            if let Some(e) = error() {
                div {
                    class: "attach__wrapper attach__wrapper--error",
                    div { class: "attach__error__header",
                        h4 { class: "attach__error__title",
                            { translate!(i18, "errors.attach.title") }
                        }
                        div {
                            class: "attach__close",
                            IconButton {
                                variant: IconButtonVariant::Round,
                                size: ElementSize::Big,
                                class: "button--avatar bg--transparent",
                                body: rsx!(
                                    Icon {
                                        icon: Close,
                                        height: 28,
                                        width: 28,
                                        fill: "var(--state-destructive-active)"
                                    }
                                ),
                                on_click: move |_| {
                                    error.set(None)
                                }
                            }
                        }
                    }
                    p { class: "attach__error__explanation",
                        "{e.explanation}"
                    }
                    p { class: "attach__error__details",
                        "{e.details}"
                    }
                }
            } else {
                div {
                    class: "attach__wrapper",
                    {
                        attach.get_file().ok().map(|url| {
                            rsx!(
                                img {
                                    class: "attach__preview",
                                    src: "{url}"
                                }
                            )
                        })
                    }

                    div {
                        class: "attach__cta",
                        Button {
                            text: "{props.cta_text}",
                            status: None,
                            variant: ButtonVariant::Secondary,
                            on_click: on_handle_attach
                        }
                    }
                }
            }
            input {
                r#type: "file",
                class: "attach__input",
                onmounted: move |event| {
                    event.data.downcast::<web_sys::Element>()
                        .and_then(|element| element.clone().dyn_into::<web_sys::HtmlElement>().ok())
                        .map(|html_element| textarea_ref.set(Some(Box::new(html_element.clone()))));
                },
                oninput: on_handle_input
            }
        }
    )
}
