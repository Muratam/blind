// use prpr::*;
use wasm_bindgen::prelude::*;
extern crate wasm_bindgen;

mod html;
mod js;
pub mod world;

#[wasm_bindgen(start)]
pub fn start() {
  world::create();
}
