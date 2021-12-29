use prpr::*;

// for entry_point
pub use wasm_bindgen::prelude::wasm_bindgen as entry_point;
pub extern crate wasm_bindgen;

#[macro_use]
extern crate assert_type_eq;

// for publish
pub mod html;
pub mod js;
pub mod prgl;
pub mod system;
use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak};
pub use system::{Time, Updatable, Updater};

// 最終的にhoge_clientに逃がす
mod sample;
pub fn run_sample() {
  js::console::log("create prpr world !!");
  system::run::<sample::SampleSystem>();
}
