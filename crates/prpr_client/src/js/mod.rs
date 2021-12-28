// pure js
use crate::html;
use wasm_bindgen::closure::Closure;
// use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn start_animation_frame_loop(mut a: Box<dyn FnMut()>) {
  fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    html::window()
      .request_animation_frame(f.as_ref().unchecked_ref())
      .expect("should register `requestAnimationFrame` OK");
  }
  let f = std::sync::Arc::new(std::cell::RefCell::new(None));
  let g = f.clone();
  *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    a();
    request_animation_frame(f.borrow().as_ref().unwrap());
  }) as Box<dyn FnMut()>));
  request_animation_frame(g.borrow().as_ref().unwrap());
}

pub mod date {
  pub fn now_millisec() -> f64 {
    js_sys::Date::now()
  }
}

pub mod console {
  #[allow(dead_code)]
  pub fn log<T: Into<wasm_bindgen::JsValue>>(value: T) {
    web_sys::console::log_1(&value.into());
  }
  #[allow(dead_code)]
  pub fn error<T: Into<wasm_bindgen::JsValue>>(value: T) {
    web_sys::console::error_1(&value.into());
  }
  #[allow(dead_code)]
  pub fn warning<T: Into<wasm_bindgen::JsValue>>(value: T) {
    web_sys::console::warn_1(&value.into());
  }
  #[allow(dead_code)]
  pub fn info<T: Into<wasm_bindgen::JsValue>>(value: T) {
    web_sys::console::info_1(&value.into());
  }
  #[allow(dead_code)]
  pub fn debug<T: Into<wasm_bindgen::JsValue>>(value: T) {
    web_sys::console::debug_1(&value.into());
  }
}
