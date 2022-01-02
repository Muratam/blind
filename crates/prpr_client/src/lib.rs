use prpr::*;
use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak};
#[macro_use]
extern crate assert_type_eq;
extern crate downcast;
// for entry_point
pub use wasm_bindgen::prelude::wasm_bindgen as entry_point;
pub extern crate wasm_bindgen;

// for publish
pub mod js;
mod owner;
pub mod prgl;
pub mod prhtml;
pub mod sample;
pub mod system;
pub use owner::*;
pub use prhtml::traits::*;
pub use system::{input, NeedUpdate, Time, Updater};
pub use system::{Why, WhyTrait, Whys};
