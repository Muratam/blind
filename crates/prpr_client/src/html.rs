// HTML
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
pub fn append_tag(parent: &web_sys::HtmlElement, tag: &str) -> web_sys::Node {
  let created = document().create_element(tag).unwrap();
  parent
    .append_child(&created)
    .expect(&format!("failed to append child ({})", tag))
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
pub fn create_root() -> web_sys::HtmlDivElement {
  append_div(&body())
}

// Canvas
pub trait Canvas {
  fn get_2d_context(&self) -> web_sys::CanvasRenderingContext2d;
}
impl Canvas for web_sys::HtmlCanvasElement {
  fn get_2d_context(&self) -> web_sys::CanvasRenderingContext2d {
    let context = self
      .get_context("2d")
      .expect("failed to get context 2d")
      .expect("failed to get context 2d");
    wasm_bindgen::JsCast::dyn_into::<web_sys::CanvasRenderingContext2d>(context)
      .expect("failed to cast to CanvasRenderingContext2d")
  }
}
