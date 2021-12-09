use prpr::*;
use wasm_bindgen::prelude::*;
extern crate wasm_bindgen;

mod html;
mod js;
pub mod scene;

#[wasm_bindgen(start)]
pub fn start() {
  scene::create_fullscreen_3d();
}
