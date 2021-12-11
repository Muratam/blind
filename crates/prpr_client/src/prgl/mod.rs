// WebGlをラップしたもの
mod renderpass;
pub use self::renderpass::*;
mod pipeline;
pub use self::pipeline::*;
mod instance;
pub use self::instance::*;
// mod raw_type;
// use self::raw_type::*;
use crate::html;
use crate::system::log;
use prpr::math::*;
use std::rc::Rc;
use web_sys::WebGl2RenderingContext as gl;
use web_sys::WebGl2RenderingContext as WebGlContext;
pub struct Texture {
  gl: Rc<WebGlContext>,
}
pub struct Buffer {
  gl: Rc<WebGlContext>,
}
pub const MAX_OUTPUT_SLOT: usize = 8;
