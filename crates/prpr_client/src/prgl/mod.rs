// WebGlをラップしたもの
mod renderpass;
pub use self::renderpass::*;
mod pipeline;
pub use self::pipeline::*;
mod instance;
pub use self::instance::*;
mod raw;
use self::raw::*;

use crate::html;
use crate::system::log;
use prpr::math::*;
use std::rc::Rc;
use web_sys::WebGl2RenderingContext as gl;
use web_sys::WebGl2RenderingContext as GlContext;

pub struct Texture {
  gl: Rc<GlContext>,
}
pub struct Buffer {
  gl: Rc<GlContext>,
}
pub const MAX_OUTPUT_SLOT: usize = 8;
