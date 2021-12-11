use super::*;
// for renderpass types
// struct RawFrameBuffer {}
// struct RawRenderBuffer {}
// struct RawColorRenderTarget {
//   texture: Option<RawTexture>,
//   frame_buffer: RawFrameBuffer,
//   clear_value: Vec4,
//   is_clear: bool,
// }
// struct RawDepthRenderTarget {
//   texture: Option<RawTexture>,
//   clear_value: f32,
//   is_clear: bool,
// }
// struct RawStencilRenderTarget {
//   texture: Option<RawTexture>,
//   clear_value: i64,
//   is_clear: bool,
// }
// struct RawRenderPass {
//   color_attachments: Vec<RawColorRenderTarget>,
//   depth_attachment: RawDepthRenderTarget,
//   stencil_attachment: RawStencilRenderTarget,
//   frame_buffer: RawFrameBuffer,
//   render_buffer: RawRenderBuffer,
//   viewport: Option<Rect<f32>>,
//   scissor: Option<Rect<i32>>,
// }

pub struct RenderPass {
  gl: Rc<WebGlContext>,
  clear_colors: [Vec4; MAX_OUTPUT_SLOT],
}
impl RenderPass {
  pub fn new(gl: Rc<WebGlContext>) -> RenderPass {
    RenderPass {
      gl: Rc::clone(&gl),
      clear_colors: [Vec4::ZERO; MAX_OUTPUT_SLOT],
    }
  }
  pub fn bind(&self) {
    let gl = &self.gl;
    // TODO: 今はゼロスロット目のみなのをなおす
    let color = self.clear_colors[0];
    gl.clear_color(color.x, color.y, color.z, color.w);
    gl.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
  }
  pub fn set_color_target(&mut self, target: &Texture) {
    log::error("set_color_target: not implemented");
  }
  pub fn set_color_target_by_slot(&mut self, target: &Texture, slot: i32) {
    log::error("set_color_target_by_slot: not implemented");
  }
  pub fn set_clear_color(&mut self, clear_color: Vec4) {
    self.set_clear_color_by_slot(clear_color, 0);
  }
  pub fn set_clear_color_by_slot(&mut self, clear_color: Vec4, slot: i32) {
    if slot < 0 || slot >= MAX_OUTPUT_SLOT as i32 {
      log::error(format!("Invalid SetClearColor Slot {}", slot));
      return;
    }
    self.clear_colors[slot as usize] = clear_color;
  }
}
