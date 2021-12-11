// WebGlをラップしたもの
mod renderpass;
pub use self::renderpass::*;
mod instance;
pub use self::instance::*;
// mod raw_type;
// use self::raw_type::*;
use crate::html;
use crate::system::log;
use prpr::math::*;
use std::rc::Rc;
use web_sys::WebGlRenderingContext as gl;
use web_sys::WebGlRenderingContext as WebGlContext;
pub struct Texture {
  gl: Rc<WebGlContext>,
}
pub struct Buffer {
  gl: Rc<WebGlContext>,
}
pub const MAX_OUTPUT_SLOT: usize = 8;
pub struct Pipeline {
  gl: Rc<WebGlContext>,
}
impl Pipeline {
  pub fn draw(&self) {}
}
