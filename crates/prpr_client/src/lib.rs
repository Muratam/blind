// use prpr::*;

// for entry_point
pub use wasm_bindgen::prelude::wasm_bindgen as entry_point;
pub extern crate wasm_bindgen;

#[macro_use]
extern crate assert_type_eq;

// for publish
mod html;
mod js;
mod prgl;
mod system;
pub use prgl::*;
use std::sync::{Arc, Mutex, RwLock};
pub use system::{run, Core, System};

// 最終的にhoge_clientに逃がす
mod sample;
pub fn run_sample() {
  js::console::log("create prpr world !!");
  run::<sample::SampleSystem>();
}
