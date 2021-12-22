use super::*;

pub struct RenderPass {
  ctx: ArcGlContext,
  clear_colors: [Option<Vec4>; MAX_OUTPUT_SLOT],
  clear_depth: Option<f32>,
  clear_stencil: Option<i32>,
  raw_framebuffer: RawFrameBuffer,
  raw_renderbuffer: RawRenderBuffer,
  // viewport: Option<Rect<f32>>,
  // scissor: Option<Rect<i32>>,
  color_targets: Vec<Option<Arc<Texture>>>,
  depth_target: Option<Arc<Texture>>,
  // stencil_target: RawStencRawTextureilRenderTarget,
}
impl RenderPass {
  pub fn new(ctx: &ArcGlContext) -> Self {
    Self {
      ctx: ctx.clone(),
      raw_framebuffer: RawFrameBuffer::new(ctx),
      raw_renderbuffer: RawRenderBuffer::new(ctx),
      clear_colors: [None; MAX_OUTPUT_SLOT],
      clear_depth: None,
      clear_stencil: None,
      color_targets: vec![None; MAX_OUTPUT_SLOT],
      depth_target: None,
    }
  }
  pub fn bind(&self) {
    let ctx = &self.ctx;
    // surface := None buffer
    // bind framebuffer
    let framebuffer = self.raw_framebuffer.raw_framebuffer();
    ctx.bind_framebuffer(gl::FRAMEBUFFER, Some(framebuffer));
    let mut color_attachment_indices = Vec::new();
    for i in 0..MAX_OUTPUT_SLOT {
      if let Some(texture) = &self.color_targets[i] {
        texture.bind();
        let color_attachment_index = index_to_color_attachments_enum(i);
        ctx.framebuffer_texture_2d(
          gl::FRAMEBUFFER,
          color_attachment_index,
          texture.target(),
          Some(texture.raw_texture().raw_texture()),
          0, // must be 0
        );
        color_attachment_indices.push(color_attachment_index);
      }
    }
    if let Some(texture) = &self.depth_target {
      texture.bind();
      ctx.framebuffer_texture_2d(
        gl::FRAMEBUFFER,
        gl::DEPTH_ATTACHMENT,
        texture.target(),
        Some(texture.raw_texture().raw_texture()),
        0, // must be 0
      );
    }

    use wasm_bindgen::JsValue;
    if let Some(buffers) = JsValue::from_serde(color_attachment_indices.as_slice()).ok() {
      ctx.draw_buffers(&buffers);
    }
    // let mut max_width = 0;
    // let mut max_height = 0;
    // max_width = std::cmp::max(max_width, texture.width());
    // max_height = std::cmp::max(max_height, texture.height());
    // let renderbuffer = self.raw_renderbuffer.raw_renderbuffer();
    // ctx.bind_renderbuffer(gl::RENDERBUFFER, Some(renderbuffer));
    // ctx.renderbuffer_storage(
    //   gl::RENDERBUFFER,
    //   gl::DEPTH_COMPONENT16,
    //   max_width as i32,
    //   max_height as i32,
    // );
    // ctx.framebuffer_renderbuffer(
    //   gl::FRAMEBUFFER,
    //   gl::DEPTH_ATTACHMENT,
    //   gl::RENDERBUFFER,
    //   Some(self.raw_renderbuffer.raw_renderbuffer()),
    // );

    // clear flag
    let mut clear_flag = 0;
    for i in 0..MAX_OUTPUT_SLOT {
      if let Some(color) = self.clear_colors[i] {
        ctx.clear_color(color.x, color.y, color.z, color.w);
        clear_flag |= gl::COLOR_BUFFER_BIT;
      }
    }
    if let Some(depth) = self.clear_depth {
      ctx.clear_depth(depth);
      clear_flag |= gl::DEPTH_BUFFER_BIT;
    }
    if let Some(stencil) = self.clear_stencil {
      ctx.clear_stencil(stencil);
      clear_flag |= gl::STENCIL_BUFFER_BIT;
    }
    if clear_flag != 0 {
      ctx.clear(clear_flag);
    }

    if SET_BIND_NONE_AFTER_WORK {
      // バインドしたままにする必要がある
      // ctx.bind_framebuffer(gl::FRAMEBUFFER, None);
      // ctx.bind_renderbuffer(gl::RENDERBUFFER, None);
      ctx.bind_texture(gl::TEXTURE_2D, None); // 違うケースがありえはする
    }
  }
  pub fn set_color_target(&mut self, target: Option<&Arc<Texture>>) {
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
  pub fn set_color_target_by_slot(&mut self, target: Option<&Arc<Texture>>, slot: i32) {
    if slot < 0 || slot >= MAX_OUTPUT_SLOT as i32 {
      log::error(format!("Invalid set_color_target_by_slot {}", slot));
      return;
    }
    if let Some(target) = target {
      self.color_targets[slot as usize] = Some(Arc::clone(target));
    } else {
      self.color_targets[slot as usize] = None;
    }
  }
  pub fn set_clear_color_by_slot(&mut self, value: Option<Vec4>, slot: i32) {
    if slot < 0 || slot >= MAX_OUTPUT_SLOT as i32 {
      log::error(format!("Invalid set_clear_color_by_slot {}", slot));
      return;
    }
    self.clear_colors[slot as usize] = value;
  }
}
