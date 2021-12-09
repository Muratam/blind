use crate::html;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

// #[macro_export] macro_rules! start_main_loop { ( $( $x:expr )? ) => { $(js::start_main_loop(Box::new($x)))*  };}
pub fn start_animation_frame_loop(mut a: Box<dyn FnMut()>) {
  fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    html::window()
      .request_animation_frame(f.as_ref().unchecked_ref())
      .expect("should register `requestAnimationFrame` OK");
  }
  let f = Rc::new(RefCell::new(None));
  let g = f.clone();
  *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    a();
    request_animation_frame(f.borrow().as_ref().unwrap());
  }) as Box<dyn FnMut()>));
  request_animation_frame(g.borrow().as_ref().unwrap());
}

pub mod console {
  pub fn log<T: Into<wasm_bindgen::JsValue>>(value: T) {
    web_sys::console::log_1(&value.into());
  }
  pub fn error<T: Into<wasm_bindgen::JsValue>>(value: T) {
    web_sys::console::error_1(&value.into());
  }
}
