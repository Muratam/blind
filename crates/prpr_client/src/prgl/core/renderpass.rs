use super::*;

struct BufferSetupInfo {
  pub is_dirty: bool,
  pub viewport: Option<Rect<i32>>, // ターゲットなしならBuffer=None
  pub use_default_buffer: bool,
}

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
  raw_framebuffer: RawFrameBuffer,
  // raw_renderbuffer: RawRenderBuffer,
  buffer_setup_info: RwLock<BufferSetupInfo>,
  descriptor: Descriptor,
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
      // raw_renderbuffer: RawRenderBuffer::new(ctx),
      buffer_setup_info: RwLock::new(BufferSetupInfo {
        is_dirty: true,
        viewport: None,
        use_default_buffer: false,
      }),
      descriptor: Descriptor::new(),
    }
  }
  fn setup_framebuffer_impl(&self) {
    let mut setup_info = self.buffer_setup_info.write().unwrap();
    if !setup_info.is_dirty {
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
      texture.raw_texture().bind();
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
    // let renderbuffer = self.raw_renderbuffer.raw_renderbuffer();
    // ctx.bind_renderbuffer(gl::RENDERBUFFER, Some(renderbuffer));
    // ctx.renderbuffer_storage(
    //   gl::RENDERBUFFER,
    //   gl::DEPTH_COMPONENT16,
    //   max_width,
    //   max_height,
    // );
    // ctx.framebuffer_renderbuffer(
    //   gl::FRAMEBUFFER,
    //   gl::DEPTH_ATTACHMENT,
    //   gl::RENDERBUFFER,
    //   Some(renderbuffer),
    // );

    if SET_BIND_NONE_AFTER_WORK {
      ctx.bind_framebuffer(gl::FRAMEBUFFER, None);
      // ctx.bind_renderbuffer(gl::RENDERBUFFER, None);
    }

    setup_info.is_dirty = false;
    setup_info.viewport = if bind_count > 0 {
      Some(Rect::new(0, 0, max_width, max_height))
    } else {
      None
    }
  }

  fn bind_framebuffer_impl(&self) {
    let ctx = &self.ctx;
    let info = &self.buffer_setup_info.read().unwrap();
    if info.viewport.is_some() {
      if info.use_default_buffer {
        log::error("[uses default framebuffer] && [has color target]");
      }
      let framebuffer = self.raw_framebuffer.raw_framebuffer();
      ctx.bind_framebuffer(gl::FRAMEBUFFER, Some(framebuffer));
      // let renderbuffer = self.raw_renderbuffer.raw_renderbuffer();
      // ctx.bind_renderbuffer(gl::RENDERBUFFER, Some(renderbuffer));
    } else if info.use_default_buffer {
      ctx.bind_framebuffer(gl::FRAMEBUFFER, None);
      // ctx.bind_renderbuffer(gl::RENDERBUFFER, None);
    } else {
      log::error("[not use default framebuffer] && [no color target]");
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
      // 設定されているなら使用
      self.ctx.viewport(v.x, v.y, v.width, v.height);
    } else if let Some(v) = &self.buffer_setup_info.read().unwrap().viewport {
      // 描画先があるならその最大サイズに
      self.ctx.viewport(v.x, v.y, v.width, v.height);
    } else {
      log::error("no renderpass viewport size (unstable)");
    }
  }

  pub fn bind(&self) -> DescriptorContext {
    self.setup_framebuffer_impl();
    self.bind_framebuffer_impl();
    self.viewport_impl();
    self.clear_impl();
    DescriptorContext::Nil.cons(&self.descriptor)
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
  pub fn set_use_default_buffer(&mut self, use_default_buffer: bool) {
    let mut info = self.buffer_setup_info.write().unwrap();
    info.use_default_buffer = use_default_buffer;
  }
  pub fn set_color_target_by_slot(&mut self, target: Option<&Arc<Texture>>, slot: i32) {
    if slot < 0 || slot >= MAX_OUTPUT_SLOT as i32 {
      log::error(format!("Invalid set_color_target_by_slot {}", slot));
      return;
    }
    self.color_targets[slot as usize] = target.map(|target| target.clone());
    self.buffer_setup_info.write().unwrap().is_dirty = true;
  }
  pub fn set_clear_color_by_slot(&mut self, value: Option<Vec4>, slot: i32) {
    if slot < 0 || slot >= MAX_OUTPUT_SLOT as i32 {
      log::error(format!("Invalid set_clear_color_by_slot {}", slot));
      return;
    }
    self.clear_colors[slot as usize] = value;
  }
  pub fn add_uniform_buffer_trait(&mut self, buffer: &Arc<dyn UniformBufferTrait>) {
    self.descriptor.add_uniform_buffer(&buffer.clone());
  }
  pub fn add_uniform_buffer<T: BufferAttribute + 'static>(
    &mut self,
    buffer: &Arc<UniformBuffer<T>>,
  ) {
    self.add_uniform_buffer_trait(&(buffer.clone() as Arc<dyn UniformBufferTrait>));
  }
  pub fn add_into_uniform_buffer<T: BufferAttribute + 'static, I: RefInto<T> + 'static>(
    &mut self,
    buffer: &Arc<IntoUniformBuffer<T, I>>,
  ) {
    self.add_uniform_buffer_trait(&(buffer.clone() as Arc<dyn UniformBufferTrait>));
  }
  pub fn add_texture_mapping<T: TextureMappingAttribute + 'static>(
    &mut self,
    mapping: &Arc<TextureMapping<T>>,
  ) {
    self
      .descriptor
      .add_texture_mapping(&(Arc::clone(mapping) as Arc<dyn TextureMappingTrait>));
  }
  pub fn add(&mut self, bindable: &dyn RenderPassBindable) {
    bindable.bind_renderpass(self);
  }
}

pub trait RenderPassBindable {
  fn bind_renderpass(&self, renderpass: &mut RenderPass);
}
