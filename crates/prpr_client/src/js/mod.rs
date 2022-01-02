use super::*;
pub mod console;
pub mod html;
use wasm_bindgen::closure::Closure;
// use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn start_animation_frame_loop(mut a: Box<dyn FnMut()>) {
  fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    html::window()
      .request_animation_frame(f.as_ref().unchecked_ref())
      .expect("should register `requestAnimationFrame` OK");
  }
  let f = Arc::new(RwLock::new(None));
  let g = f.clone();
  *g.write().unwrap() = Some(Closure::wrap(Box::new(move || {
    a();
    request_animation_frame(f.read().unwrap().as_ref().unwrap());
  }) as Box<dyn FnMut()>));
  request_animation_frame(g.read().unwrap().as_ref().unwrap());
}
