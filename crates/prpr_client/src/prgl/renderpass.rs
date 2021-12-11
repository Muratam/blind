use super::*;

pub struct RenderPass {
  gl: Rc<GlContext>,
  clear_colors: [Option<Vec4>; MAX_OUTPUT_SLOT],
  clear_depth: Option<f32>,
  clear_stencil: Option<i32>,
  // frame_buffer: RawFrameBuffer,
  // render_buffer: RawRenderBuffer,
  // viewport: Option<Rect<f32>>,
  // scissor: Option<Rect<i32>>,
  // color_targets: Vec<RawTexture>,
  // depth_target: RawTexture,
  // stencil_target: RawStencRawTextureilRenderTarget,
}
impl RenderPass {
  pub fn new(gl: Rc<GlContext>) -> Self {
    Self {
      gl: Rc::clone(&gl),
      clear_colors: [None; MAX_OUTPUT_SLOT],
      clear_depth: None,
      clear_stencil: None,
    }
  }
  pub fn bind(&self) {
    let gl = &self.gl;
    // TODO: 今はゼロスロット目のみ. 今はテクスチャバインドなし
    let mut flag = 0;
    if let Some(color) = self.clear_colors[0] {
      gl.clear_color(color.x, color.y, color.z, color.w);
      flag |= gl::COLOR_BUFFER_BIT;
    }
    if let Some(depth) = self.clear_depth {
      gl.clear_depth(depth);
      flag |= gl::DEPTH_BUFFER_BIT;
    }
    if let Some(stencil) = self.clear_stencil {
      gl.clear_stencil(stencil);
      flag |= gl::STENCIL_BUFFER_BIT;
    }
    gl.clear(flag);
  }
  pub fn set_color_target(&mut self, target: &Texture) {
    self.set_color_target_by_slot(target, 0);
  }
  pub fn set_clear_color(&mut self, value: Option<Vec4>) {
    self.set_clear_color_by_slot(value, 0);
  }
  pub fn set_clear_depth(&mut self, value: Option<f32>) {
    self.clear_depth = value;
  }
  pub fn set_clear_stencil(&mut self, value: Option<i32>) {
    self.clear_stencil = value;
  }
  pub fn set_color_target_by_slot(&mut self, target: &Texture, slot: i32) {
    log::error("set_color_target_by_slot: not implemented");
  }
  pub fn set_clear_color_by_slot(&mut self, value: Option<Vec4>, slot: i32) {
    if slot < 0 || slot >= MAX_OUTPUT_SLOT as i32 {
      log::error(format!("Invalid SetClearColor Slot {}", slot));
      return;
    }
    self.clear_colors[slot as usize] = value;
  }
}
