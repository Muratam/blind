extern crate wasm_bindgen;
extern crate web_sys;
use prpr::*;
use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::*;

// #[wasm_bindgen] extern "C" { pub fn alert(s: &str); }
pub mod console {
  pub fn log<T: Into<wasm_bindgen::JsValue>>(value: T) {
    web_sys::console::log_1(&value.into());
  }
}
mod html {
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
}

trait Canvas {
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

mod canvas {
  pub fn render_sample(context: &web_sys::CanvasRenderingContext2d) {
    use std::f64::consts::PI;
    context.begin_path();
    context.arc(75.0, 75.0, 50.0, 0.0, PI * 2.0).ok();
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, PI).ok();
    context.move_to(65.0, 65.0);
    context.arc(60.0, 65.0, 5.0, 0.0, PI * 2.0).ok();
    context.move_to(95.0, 65.0);
    context.arc(90.0, 65.0, 5.0, 0.0, PI * 2.0).ok();
    context.stroke();
  }
}

#[wasm_bindgen(start)]
pub fn start() {
  let root = html::create_root();
  let canvas = html::append_canvas(&root);
  let context = canvas.get_2d_context();
  canvas::render_sample(&context);
  console::log(&context);
  // root.set_text_content(Some("Hello from Rust!"));
  console::log("abc");
  console::log(&root);
  console::log(1 + 2);
}
