extern crate wasm_bindgen;
extern crate web_sys;
use prpr::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  pub fn alert(s: &str);
}
fn greet(name: &str) {
  alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn entry_point() {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let body = document.body().expect("document should have a body");
  let root = document.create_element("div").unwrap();
  root.set_text_content(Some("Hello from Rust!"));
  body.append_child(&root).unwrap();
  // greet("abc");
}
