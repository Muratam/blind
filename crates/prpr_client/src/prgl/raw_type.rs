mod todo_move_to_prpr {
  pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
  }
  pub struct Rect<T> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
  }
}
use todo_move_to_prpr::*;

// 実態として必要な情報を全て詰め込んだもの
pub use web_sys::WebGlRenderingContext as Gl;
pub const SWAP_COUNT: i64 = 2;
struct RawBuffer {
  need_swap: bool,
}
struct RawTexture {
  need_swap: bool,
}
struct RawSampler {}
struct RawExtensions {}
struct RawShader {}
enum RawDrawCommand {}
struct RawPipeline {
  is_enabled_depth_test: bool,
  draw_command: RawDrawCommand,
  // gl.depth_func(gl::LEQUAL);
}

// for renderpass
struct RawFrameBuffer {}
struct RawRenderBuffer {}
struct RawColorRenderTarget {
  texture: Option<RawTexture>,
  frame_buffer: RawFrameBuffer,
  clear_value: Vec4,
  is_clear: bool,
}
struct RawDepthRenderTarget {
  texture: Option<RawTexture>,
  clear_value: f64,
  is_clear: bool,
}
struct RawStencilRenderTarget {
  texture: Option<RawTexture>,
  clear_value: i64,
  is_clear: bool,
}
struct RawRenderPass {
  color_attachments: Vec<RawColorRenderTarget>,
  depth_attachment: RawDepthRenderTarget,
  stencil_attachment: RawStencilRenderTarget,
  frame_buffer: RawFrameBuffer,
  render_buffer: RawRenderBuffer,
  viewport: Rect<f64>,
  scissor: Rect<i64>,
}
