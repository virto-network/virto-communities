use crate::components::atoms::button::Variant;
use crate::components::atoms::dropdown::ElementSize;
use crate::components::atoms::Button;
use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::js_sys::Function;
use web_sys::HtmlElement;
#[derive(PartialEq, Props, Clone)]
pub struct MarkdownEvent {
    pub value: String,
}
#[derive(PartialEq, Props, Clone)]
pub struct MarkdownProps {
    #[props(default = "".to_string())]
    class: String,
    content: String,
    on_input: EventHandler<MarkdownEvent>,
}
#[wasm_bindgen(inline_js = r#"
    let tinyEditor;
    export function initMarkdownEditor(editorElement, toolbarElement, onChangeCallback) {       
        tinyEditor = new TinyMDE.Editor({ element: editorElement });
        let commandBar = new TinyMDE.CommandBar({
            element: toolbarElement,
            editor: tinyEditor,
        });

        tinyEditor.addEventListener('change', function () {
            let content = tinyEditor.getContent();
            onChangeCallback(content);
        });
    }

    export function setContentMarkdownEditor(content) {
        tinyEditor.setContent(content);
    }
"#)]
extern "C" {
    #[wasm_bindgen(js_name = initMarkdownEditor)]
    fn init_markdown_editor(editor: HtmlElement, toolbar: HtmlElement, callback: &Function);
    #[wasm_bindgen(js_name = setContentMarkdownEditor)]
    fn set_content_markdown_editor(content: String);
}
pub fn Markdown(props: MarkdownProps) -> Element {
    let i18 = use_i18();
    let mut is_editor_loaded = use_signal(|| false);
    let content = use_signal(|| {
        if props.content.len() > 0 {
            props.content.clone()
        } else {
            translate!(i18, "utils.markdown.value")
        }
    });
    let mut is_markdown_visible = use_signal(|| true);
    let mut toolbar_ref = use_signal::<Option<Box<HtmlElement>>>(|| None);
    let mut editor_ref = use_signal::<Option<Box<HtmlElement>>>(|| None);
    let cont = props.content;
    let parser = pulldown_cmark::Parser::new(&cont);
    let mut html_buf = String::new();
    pulldown_cmark::html::push_html(&mut html_buf, parser);
    use_effect(move || {
        if !is_editor_loaded() {
            if let Some(toolbar_ref) = toolbar_ref() {
                if let Some(editor_ref) = editor_ref() {
                    let closure = Closure::wrap(Box::new(move |new_content: JsValue| {
                        if let Some(text) = new_content.as_string() {
                            props.on_input.call(MarkdownEvent { value: text })
                        }
                    }) as Box<dyn FnMut(JsValue)>);
                    let function = closure.as_ref().unchecked_ref::<Function>();
                    init_markdown_editor(*editor_ref.clone(), *toolbar_ref.clone(), function);
                    set_content_markdown_editor(content());
                    closure.forget();
                    is_editor_loaded.set(true);
                }
            }
        }
    });
    rsx!(
        div { class: "markdown",
            div {
                class: "markdown__wrapper",
                class: if !is_markdown_visible() { "hide" } else { "markdown__wrapper--editor" },
                div {
                    onmounted: move |event| {
                        event
                            .data
                            .downcast::<web_sys::Element>()
                            .and_then(|element| element.clone().dyn_into::<web_sys::HtmlElement>().ok())
                            .map(|html_element| toolbar_ref.set(Some(Box::new(html_element.clone()))));
                    },
                }
                div {
                    onmounted: move |event| {
                        event
                            .data
                            .downcast::<web_sys::Element>()
                            .and_then(|element| element.clone().dyn_into::<web_sys::HtmlElement>().ok())
                            .map(|html_element| editor_ref.set(Some(Box::new(html_element.clone()))));
                    },
                }
            }
            div {
                class: "markdown__wrapper",
                class: if !is_markdown_visible() { "markdown__wrapper--preview" } else { "hide" },
                dangerous_inner_html: "{html_buf}"
            }
            div {
                Button {
                    class: "",
                    text: if !is_markdown_visible() {
                        translate!(i18, "utils.markdown.cta.edit")
                    } else {
                        translate!(i18, "utils.markdown.cta.preview")
                    },
                    size: ElementSize::Small,
                    variant: Variant::Tertiary,
                    on_click: move |_| {
                        is_markdown_visible.toggle();
                    },
                    status: None
                }
            }
        }
    )
}
