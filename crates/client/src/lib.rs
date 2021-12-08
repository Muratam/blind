extern crate wasm_bindgen;
extern crate web_sys;
use prpr_anywhere::*;
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
  let document = web_sys::window().unwrap().document().unwrap();
  let root = document.get_element_by_id(HTML_ROOT_DIV_ID).unwrap();
  root.set_inner_html("aaaaaaaa");
}
