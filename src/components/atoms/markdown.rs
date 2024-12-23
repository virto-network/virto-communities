use crate::components::atoms::button::Variant;
use crate::components::atoms::dropdown::ElementSize;
use crate::components::atoms::Button;
use dioxus::prelude::*;
use dioxus_i18n::t;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::js_sys;
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
    export function initMarkdownEditor(editorElement, toolbarElement, onChangeCallback) {       
        let tinyEditor = new TinyMDE.Editor({ element: editorElement });
        let commandBar = new TinyMDE.CommandBar({
            element: toolbarElement,
            editor: tinyEditor,
        });

        tinyEditor.addEventListener('change', function () {
            let content = tinyEditor.getContent();
            onChangeCallback(content);
        });

        return tinyEditor;
    }
"#)]
extern "C" {
    #[wasm_bindgen(js_name = initMarkdownEditor)]
    fn init_markdown_editor(
        editor: HtmlElement,
        toolbar: HtmlElement,
        callback: &Function,
    ) -> JsValue;
}

fn call_method_reflect(
    obj: &JsValue,
    method_name: &str,
    args: &[JsValue],
) -> Result<JsValue, JsValue> {
    let method = js_sys::Reflect::get(obj, &method_name.into())?;
    let func: js_sys::Function = method.dyn_into()?;
    func.apply(obj, &js_sys::Array::from_iter(args))
}

pub fn Markdown(props: MarkdownProps) -> Element {
    let mut is_editor_loaded = use_signal(|| false);
    // let content = use_signal(|| {
    //     if !props.content.is_empty() {
    //         props.content.clone()
    //     } else {
    //         t!("utils-markdown-value")
    //     }
    // });
    let content = use_signal(String::new);
    let mut is_markdown_visible = use_signal(|| true);
    let mut toolbar_ref = use_signal::<Option<Box<HtmlElement>>>(|| None);
    let mut editor_ref = use_signal::<Option<Box<HtmlElement>>>(|| None);
    let cont = props.content;
    let parser = pulldown_cmark::Parser::new(&cont);
    let mut html_buf = String::new();
    pulldown_cmark::html::push_html(&mut html_buf, parser);
    use_effect(move || {
        if !*is_editor_loaded.read() {
            if let (Some(toolbar_ref), Some(editor_ref)) = (toolbar_ref(), editor_ref()) {
                let closure = Closure::wrap(Box::new(move |new_content: JsValue| {
                    if let Some(text) = new_content.as_string() {
                        props.on_input.call(MarkdownEvent { value: text });
                    }
                }) as Box<dyn FnMut(JsValue)>);
                let function = closure.as_ref().unchecked_ref::<Function>();
                let tiny_editor =
                    init_markdown_editor(*editor_ref.clone(), *toolbar_ref.clone(), function);
                let content_value = JsValue::from(content());
                if let Err(e) = call_method_reflect(&tiny_editor, "setContent", &[content_value]) {
                    dioxus::logger::tracing::warn!("Failed to set content {:?}", e);
                };

                closure.forget();
                // is_editor_loaded.set(true);
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
                        if let Some(html_element) = event
                            .data
                            .downcast::<web_sys::Element>()
                            .and_then(|element| element.clone().dyn_into::<web_sys::HtmlElement>().ok())
                        {
                            toolbar_ref.set(Some(Box::new(html_element.clone())))
                        }
                    }
                }
                div {
                    onmounted: move |event| {
                        if let Some(html_element) = event
                            .data
                            .downcast::<web_sys::Element>()
                            .and_then(|element| element.clone().dyn_into::<web_sys::HtmlElement>().ok())
                        {
                            editor_ref.set(Some(Box::new(html_element.clone())))
                        }
                    }
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
                        t!("utils-markdown-cta-edit")
                    } else {
                        t!("utils-markdown-cta-preview")
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
