pub fn window() -> web_sys::Window {
  web_sys::window().expect("no global `window` exists")
}
pub fn document() -> web_sys::Document {
  window()
    .document()
    .expect("should have a document on window")
}
pub fn body() -> web_sys::HtmlElement {
  document().body().expect("document should have a body")
}
pub fn screen() -> web_sys::Screen {
  window().screen().expect("should have a screen on window")
}

use std::sync::atomic::{AtomicUsize, Ordering};
static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
fn create_tag(tag: &str) -> web_sys::Element {
  let id = ID_COUNTER.fetch_add(1, Ordering::SeqCst) as u64;
  let created = document().create_element(tag).unwrap();
  created.set_id(&format!("prpr-id-{}", id));
  created
}
pub fn append_tag(parent: &web_sys::HtmlElement, tag: &str) -> web_sys::HtmlElement {
  let created = create_tag(tag);
  let elem = parent
    .append_child(&created)
    .expect(&format!("failed to append child ({})", tag));
  let result =
    wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlElement>(elem).expect("failed cast to div");
  result
}
pub fn append_div(parent: &web_sys::HtmlElement) -> web_sys::HtmlDivElement {
  let div = append_tag(parent, "div");
  wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlDivElement>(div).expect("failed cast to div")
}
pub fn append_span(parent: &web_sys::HtmlElement) -> web_sys::HtmlElement {
  let div = append_tag(parent, "span");
  wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlElement>(div).expect("failed cast span")
}
pub fn append_canvas(parent: &web_sys::HtmlElement) -> web_sys::HtmlCanvasElement {
  let canvas = append_tag(parent, "canvas");
  wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlCanvasElement>(canvas)
    .expect("failed cast to canvas")
}
pub fn append_css(parent: &web_sys::HtmlElement, text: &str) -> web_sys::HtmlStyleElement {
  let style = append_tag(parent, "style");
  style.set_text_content(Some(text));
  wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlStyleElement>(style).expect("failed cast to style")
}
