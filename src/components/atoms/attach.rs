use std::ops::Deref;

use dioxus::prelude::*;
use futures_util::TryFutureExt;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::{
    components::atoms::{button::Variant, Button},
    hooks::use_attach::{use_attach, AttachFile},
};

#[derive(Clone, Debug)]
pub enum AttachError {
    NotFound,
    UncoverType,
    UnknownContent,
}

#[derive(PartialEq, Debug, Clone)]
pub struct AttachEvent {
    pub value: Vec<u8>,
}

#[derive(PartialEq, Props, Clone)]
pub struct AttachProps {
    // value: Vec<u8>,
    label: Option<String>,
    cta_text: String,
}

pub fn Attach(props: AttachProps) -> Element {
    let mut textarea_ref = use_signal::<Option<Box<HtmlElement>>>(|| None);
    let mut preview_url = use_signal(|| None);
    let mut attach = use_attach();

    let on_handle_attach = move |_| {
        if let Some(input_element) = textarea_ref() {
            input_element.click();
        }
    };

    let on_handle_input = move |event: Event<FormData>| {
        spawn({
            async move {
                let files = &event.files().ok_or(AttachError::NotFound)?;
                let fs = files.files();

                let existing_file = fs.get(0).ok_or(AttachError::NotFound)?;
                let content = files
                    .read_file(existing_file)
                    .await
                    .ok_or(AttachError::NotFound)?;
                let infered_type = infer::get(content.deref()).ok_or(AttachError::UncoverType)?;

                let content_type: Result<mime::Mime, _> = infered_type.mime_type().parse();
                let content_type = content_type.map_err(|_| AttachError::UnknownContent)?;

                let blob = match content_type.type_() {
                    mime::IMAGE => gloo::file::Blob::new(content.deref()),
                    mime::VIDEO => gloo::file::Blob::new_with_options(
                        content.deref(),
                        Some(infered_type.mime_type()),
                    ),
                    _ => gloo::file::Blob::new(content.deref()),
                };

                let size = blob.size().clone();
                let object_url = gloo::file::ObjectUrl::from(blob);
                preview_url.set(Some(object_url.clone()));

                attach.set(Some(AttachFile {
                    name: existing_file.to_string(),
                    preview_url: object_url,
                    data: content.clone(),
                    content_type,
                    size,
                }));

                Ok::<(), AttachError>(())
            }
            .unwrap_or_else(move |e: AttachError| {
                let message_error = match e {
                    AttachError::NotFound => "NotFound",
                    AttachError::UncoverType => "UncoverType",
                    AttachError::UnknownContent => "UnknownContent",
                };

                log::info!("error attach: {:?}", message_error)
            })
        });
    };

    rsx!(
        section {
            class: "attach",
            if let Some(value) = props.label {
                label { class: "input__label", "{value}" }
            }

            div {
                class: "attach__wrapper",
                match preview_url() {
                    Some(url) => rsx!(
                        img {
                            class: "attach__preview",
                            src: "{url}"
                        }
                    ),
                    None => rsx!(
                        div {
                            class: "attach__preview"
                        }
                    )
                }

                div {
                    class: "attach__cta",
                    Button {
                        text: "Lo tengo",
                        status: None,
                        variant: Variant::Secondary,
                        on_click: on_handle_attach
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
