use prpr::*;
#[macro_use]
extern crate assert_type_eq;
extern crate downcast;
// for entry_point
pub use wasm_bindgen::prelude::wasm_bindgen as entry_point;
pub extern crate wasm_bindgen;

// for publish
pub mod js;
pub mod prgl;
pub mod prhtml;
pub mod sample;
pub mod system;
pub use prhtml::traits::*;
pub use prpr::owner::*;
pub use system::{input, NeedUpdate, Time, Updater};
pub use system::{Why, WhyTrait, Whys};
