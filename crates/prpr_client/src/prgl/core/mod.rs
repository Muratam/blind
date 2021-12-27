// WebGlをラップしたもの
use super::*;
pub use web_sys::WebGl2RenderingContext as gl;
pub type ArcGlContext = Arc<web_sys::WebGl2RenderingContext>;
pub const MAX_OUTPUT_SLOT: usize = 8;
pub type IndexBufferType = u32;
const SET_BIND_NONE_AFTER_WORK: bool = false;

// raw not publish
pub mod raw;
use self::raw::*;

// others publish
mod renderpass;
pub use self::renderpass::*;
mod pipeline;
pub use self::pipeline::*;
mod descriptorset;
pub use self::descriptorset::*;
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
mod vao;
pub use self::vao::*;
pub mod command;
pub use self::command::*;
