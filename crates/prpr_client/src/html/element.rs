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
pub fn append_tag(parent: &web_sys::HtmlElement, tag: &str) -> web_sys::HtmlElement {
  let created = document().create_element(tag).unwrap();
  let elem = parent
    .append_child(&created)
    .expect(&format!("failed to append child ({})", tag));
  wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlElement>(elem).expect("failed cast to div")
}
pub fn append_div(parent: &web_sys::HtmlElement) -> web_sys::HtmlDivElement {
  let div = append_tag(parent, "div");
  wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlDivElement>(div).expect("failed cast to div")
}
pub fn append_canvas(parent: &web_sys::HtmlElement) -> web_sys::HtmlCanvasElement {
  let canvas = append_tag(parent, "canvas");
  wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlCanvasElement>(canvas)
    .expect("failed cast to canvas")
}
pub fn append_css(parent: &web_sys::HtmlElement, text: &str) -> web_sys::HtmlStyleElement {
  let tag = "style";
  let created = document().create_element(tag).unwrap();
  created.set_text_content(Some(text));
  let elem = parent
    .append_child(&created)
    .expect(&format!("failed to append child ({})", tag));
  wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlStyleElement>(elem).expect("failed cast to style")
}
