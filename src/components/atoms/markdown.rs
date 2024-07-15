use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::js_sys::Function;

use crate::components::atoms::button::Variant;
use crate::components::atoms::dropdown::ElementSize;
use crate::components::atoms::Button;

#[derive(PartialEq, Props, Clone)]
pub struct MarkdownProps {
    #[props(default = "".to_string())]
    class: String,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = globalThis, js_name = initMarkdownEditor)]
    fn init_markdown_editor();

    #[wasm_bindgen(js_namespace = window, js_name = initMarkdownEditor)]
    fn add_change_listener(callback: &Function);

    #[wasm_bindgen(js_namespace = window, js_name = setContentMarkdownEditor)]
    fn set_content_markdown_editor(content: String);
}

pub fn Markdown(props: MarkdownProps) -> Element {
    let i18 = use_i18();

    let mut content = use_signal(|| translate!(i18, "utils.markdown.value"));
    let mut is_markdown_visible = use_signal(|| true);

    let cont = &*content.read();
    let parser = pulldown_cmark::Parser::new(cont);

    let mut html_buf = String::new();
    pulldown_cmark::html::push_html(&mut html_buf, parser);
    
    use_effect(move || {
        init_markdown_editor();
        set_content_markdown_editor(translate!(i18, "utils.markdown.value"));

        let closure = Closure::wrap(Box::new(move |new_content: JsValue| {
            if let Some(text) = new_content.as_string() {
                content.set(text);
            }
        }) as Box<dyn FnMut(JsValue)>);

        let function = closure.as_ref().unchecked_ref::<Function>();
        add_change_listener(function);
        closure.forget();
    });

    rsx!(
        div { class:"markdown",
            div {
                class: "markdown__wrapper",
                class: if !is_markdown_visible() {"hide"} else {"markdown__wrapper--editor"},
                div { id: "toolbar" }
                div { id: "editor" }
            }
            div {
                class: "markdown__wrapper",
                class: if !is_markdown_visible() {"markdown__wrapper--preview"} else {"hide"},
                dangerous_inner_html: "{html_buf}"
            }
            div {
                Button {
                    class: "",
                    text: if !is_markdown_visible() {translate!(i18, "utils.markdown.cta.edit")} else {translate!(i18, "utils.markdown.cta.preview")},
                    size: ElementSize::Small,
                    variant: Variant::Tertiary,
                    on_click: move |_| {
                        is_markdown_visible.toggle();
                    },
                    status: None,
                }
            }
        }
    )
}
