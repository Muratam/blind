// WebGlをラップしたもの
mod renderpass;
pub use self::renderpass::*;
mod pipeline;
pub use self::pipeline::*;
mod descriptorset;
pub use self::descriptorset::*;
mod instance;
pub use self::instance::*;
mod buffer;
pub use self::buffer::*;
mod texture;
pub use self::texture::*;
mod shader;
pub use self::shader::*;
mod template;
pub use self::template::*;
mod template_macro;
pub use self::template_macro::*;
mod state;
pub use self::state::*;
pub mod raw;
use self::raw::*;

use crate::html;
use crate::system::log;
pub use prpr::math::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use web_sys::WebGl2RenderingContext as gl;
use web_sys::WebGl2RenderingContext as GlContext;

pub const MAX_OUTPUT_SLOT: usize = 8;
type IndexBufferType = u32;
const SET_BIND_NONE_AFTER_WORK: bool = true;
