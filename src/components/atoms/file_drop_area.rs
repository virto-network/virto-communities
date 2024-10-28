use std::sync::Arc;

use dioxus::prelude::*;
use dioxus_elements::{FileEngine, HasFileData};

use crate::components::atoms::{FileListLine, Icon};

struct UploadedFile {
    name: String,
}

#[derive(PartialEq, Props, Clone)]
pub struct FileDropAreaProps {
    label: Option<String>,
    help: Option<String>,
    #[props(default = false)]
    show_files_list: bool,
}

pub fn FileDropArea(props: FileDropAreaProps) -> Element {
    let mut files_uploaded = use_signal(|| Vec::new() as Vec<UploadedFile>);
    let mut hovered = use_signal(|| false);

    let read_files = move |file_engine: Arc<dyn FileEngine>| async move {
        let files = file_engine.files();
        for file_name in &files {
            if let Some(_) = file_engine.read_file_to_string(file_name).await {
                files_uploaded.write().push(UploadedFile {
                    name: file_name.clone(),
                });
            }
        }
    };

    rsx! {
        section { class: "file-drop",
            if let Some(value) = props.label {
                label { class: "input__label",
                    "{value}"
                }
            }
            div {
                class: "drop-zone",
                id: "drop-zone",
                prevent_default: "ondragover ondrop",
                class: if hovered() { "drop-zone--hovered" },
                ondragover: move |_| hovered.set(true),
                ondragleave: move |_| hovered.set(false),
                ondrop: move |evt| async move {
                    hovered.set(false);
                    if let Some(file_engine) = evt.files() {
                        read_files(file_engine).await;
                    }
                },
                Icon { icon: FileListLine, height: 28, width: 28, fill: "var(--state-brand-primary)" }
                div { class: "drop-zone__wrapper",
                    h3 { class: "drop-zone__title", "Drag & drop bills"}
                    span { class: "drop-zone__description", "We can grab your recipient’s details automatically"}
                }
            }
            if let Some(help) = props.help {
                div { class: "input--help", "{help}" }
            }

            if props.show_files_list {
                ul {
                    for file in files_uploaded.read().iter().rev() {
                        li {
                            span { "{file.name}" }
                        }
                    }
                }
            }
        }
    }
}
