use super::*;

pub struct RenderPass {
  ctx: ArcGlContext,
  clear_colors: [Option<Vec4>; MAX_OUTPUT_SLOT],
  clear_depth: Option<f32>,
  clear_stencil: Option<i32>,
  // raw_renderbuffer: RawRenderBuffer,
  //
  // None => TargetのMax
  viewport: Option<Rect<i32>>,
  // scissor: Option<Rect<i32>>,
  //
  // None => Surface
  color_targets: Vec<Option<Arc<Texture>>>,
  depth_target: Option<Arc<Texture>>,
  // stencil_target: Option<Arc<Texture>>,
  //
  // 不要な更新は避ける
  raw_framebuffer: RawFrameBuffer,
  is_dirty_targets: Mutex<bool>,
  max_targets_rect: RwLock<Option<Rect<i32>>>, // None => No Target => Surface
}
impl RenderPass {
  pub fn new(ctx: &ArcGlContext) -> Self {
    Self {
      ctx: ctx.clone(),
      clear_colors: [None; MAX_OUTPUT_SLOT],
      clear_depth: None,
      clear_stencil: None,
      //
      viewport: None,
      //
      color_targets: vec![None; MAX_OUTPUT_SLOT],
      depth_target: None,
      //
      raw_framebuffer: RawFrameBuffer::new(ctx),
      is_dirty_targets: Mutex::new(true),
      max_targets_rect: RwLock::new(None),
    }
  }
  fn setup_framebuffer_impl(&self) {
    let mut is_dirty_targets = self.is_dirty_targets.lock().unwrap();
    if !*is_dirty_targets {
      return;
    }
    let ctx = &self.ctx;
    let framebuffer = self.raw_framebuffer.raw_framebuffer();
    ctx.bind_framebuffer(gl::FRAMEBUFFER, Some(framebuffer));
    let mut color_attachment_indices = Vec::new();
    let mut max_width: i32 = 0;
    let mut max_height: i32 = 0;
    let mut bind_count: i32 = 0;
    let mut bind_impl = |attachment: u32, texture: &Arc<Texture>| {
      texture.bind();
      ctx.framebuffer_texture_2d(
        gl::FRAMEBUFFER,
        attachment,
        texture.target(),
        Some(texture.raw_texture().raw_texture()),
        0, // must be 0
      );
      max_width = std::cmp::max(max_width, texture.width() as i32);
      max_height = std::cmp::max(max_height, texture.width() as i32);
      bind_count += 1;
      if SET_BIND_NONE_AFTER_WORK {
        ctx.bind_texture(gl::TEXTURE_2D, None);
      }
    };
    for i in 0..MAX_OUTPUT_SLOT {
      if let Some(texture) = &self.color_targets[i] {
        let color_attachment_index = index_to_color_attachments_enum(i);
        color_attachment_indices.push(color_attachment_index);
        bind_impl(color_attachment_index, &texture);
      }
    }
    if let Some(texture) = &self.depth_target {
      bind_impl(gl::DEPTH_ATTACHMENT, &texture);
    }

    use wasm_bindgen::JsValue;
    if let Some(buffers) = JsValue::from_serde(color_attachment_indices.as_slice()).ok() {
      ctx.draw_buffers(&buffers);
    }
    if SET_BIND_NONE_AFTER_WORK {
      ctx.bind_framebuffer(gl::FRAMEBUFFER, None);
    }

    *is_dirty_targets = false;
    *self.max_targets_rect.write().unwrap() = if bind_count > 0 {
      Some(Rect::new(0, 0, max_width, max_height))
    } else {
      None
    }
  }
  fn bind_framebuffer_impl(&self) {
    let ctx = &self.ctx;
    if self.max_targets_rect.read().unwrap().is_none() {
      // None => to surface
      ctx.bind_framebuffer(gl::FRAMEBUFFER, None);
    } else {
      let framebuffer = self.raw_framebuffer.raw_framebuffer();
      ctx.bind_framebuffer(gl::FRAMEBUFFER, Some(framebuffer));
    }
  }

  fn clear_impl(&self) {
    let ctx = &self.ctx;
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
  }
  fn viewport_impl(&self) {
    if let Some(v) = &self.viewport {
      // 設定されているものに
      self.ctx.viewport(v.x, v.y, v.width, v.height);
    } else if let Some(v) = &*self.max_targets_rect.read().unwrap() {
      // 描画先の最大サイズに
      self.ctx.viewport(v.x, v.y, v.width, v.height);
    } else {
      log::error("no renderpass viewport size (unstable)");
    }
  }

  pub fn bind(&self) {
    self.setup_framebuffer_impl();
    self.bind_framebuffer_impl();
    self.viewport_impl();
    self.clear_impl();
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
  pub fn set_viewport(&mut self, viewport: Option<&Rect<i32>>) {
    self.viewport = viewport.map(|v| v.clone());
  }
  pub fn set_color_target_by_slot(&mut self, target: Option<&Arc<Texture>>, slot: i32) {
    if slot < 0 || slot >= MAX_OUTPUT_SLOT as i32 {
      log::error(format!("Invalid set_color_target_by_slot {}", slot));
      return;
    }
    self.color_targets[slot as usize] = target.map(|target| target.clone());
    *self.is_dirty_targets.lock().unwrap() = true;
  }
  pub fn set_clear_color_by_slot(&mut self, value: Option<Vec4>, slot: i32) {
    if slot < 0 || slot >= MAX_OUTPUT_SLOT as i32 {
      log::error(format!("Invalid set_clear_color_by_slot {}", slot));
      return;
    }
    self.clear_colors[slot as usize] = value;
  }
}
