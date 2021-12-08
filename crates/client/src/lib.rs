extern crate wasm_bindgen;

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
  greet("PRPR");
}
