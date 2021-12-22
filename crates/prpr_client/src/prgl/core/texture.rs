use super::*;

pub struct Texture {
  ctx: ArcGlContext,
  raw_texture: RawTexture,
}
pub type Texture2dDescriptor = RawTexture2dDescriptor;
pub type PixelFormat = RawPixelFormat;
impl Texture {
  pub fn new_fill_color(ctx: &ArcGlContext, width: usize, height: usize, color: Vec4) -> Self {
    let size = width * height;
    let mut data: Vec<u8> = Vec::new();
    fn clamp(x: f32) -> u8 {
      if x > 1.0 {
        255
      } else if x < 0.0 {
        0
      } else {
        (x * 255.0) as u8
      }
    }
    let color_u8x4 = [
      clamp(color.x),
      clamp(color.y),
      clamp(color.z),
      clamp(color.w),
    ];
    for _ in 0..size {
      for color_u8 in color_u8x4 {
        data.push(color_u8);
      }
    }
    Self::new_bytes(
      ctx,
      &Texture2dDescriptor {
        width,
        height,
        format: PixelFormat::R8G8B8A8,
        mipmap: true,
      },
      data.as_slice(),
    )
  }
  pub fn new_bytes(ctx: &ArcGlContext, desc: &Texture2dDescriptor, data: &[u8]) -> Self {
    Self::new_impl(ctx, desc, TextureWriteType::u8(data))
  }
  pub fn new_floats(ctx: &ArcGlContext, desc: &Texture2dDescriptor, data: &[f32]) -> Self {
    Self::new_impl(ctx, desc, TextureWriteType::f32(data))
  }
  pub fn new_uninitialized(ctx: &ArcGlContext, desc: &Texture2dDescriptor) -> Self {
    Self::new_impl(ctx, desc, TextureWriteType::Uninitialized)
  }
  pub fn new_fill_zero(ctx: &ArcGlContext, desc: &Texture2dDescriptor) -> Self {
    Self::new_impl(ctx, desc, TextureWriteType::Zero)
  }
  pub fn new_fill_one(ctx: &ArcGlContext, desc: &Texture2dDescriptor) -> Self {
    Self::new_impl(ctx, desc, TextureWriteType::One)
  }
  pub fn new_image_bitmap(
    ctx: &ArcGlContext,
    desc: &Texture2dDescriptor,
    data: &web_sys::ImageBitmap,
  ) -> Self {
    Self::new_impl(ctx, desc, TextureWriteType::ImageBitmap(data))
  }
  pub fn new_image_data(
    ctx: &ArcGlContext,
    desc: &Texture2dDescriptor,
    data: &web_sys::ImageData,
  ) -> Self {
    Self::new_impl(ctx, desc, TextureWriteType::ImageData(data))
  }
  pub fn new_html_image_element(
    ctx: &ArcGlContext,
    desc: &Texture2dDescriptor,
    data: &web_sys::HtmlImageElement,
  ) -> Self {
    Self::new_impl(ctx, desc, TextureWriteType::HtmlImageElement(data))
  }
  pub fn new_html_canvas_element(
    ctx: &ArcGlContext,
    desc: &Texture2dDescriptor,
    data: &web_sys::HtmlCanvasElement,
  ) -> Self {
    Self::new_impl(ctx, desc, TextureWriteType::HtmlCanvasElement(data))
  }
  pub fn new_html_video_element(
    ctx: &ArcGlContext,
    desc: &Texture2dDescriptor,
    data: &web_sys::HtmlVideoElement,
  ) -> Self {
    Self::new_impl(ctx, desc, TextureWriteType::HtmlVideoElement(data))
  }
  pub fn bind(&self) {
    let target = self.raw_texture.target();
    self
      .ctx
      .bind_texture(target, Some(self.raw_texture.raw_texture()));
  }
  pub fn apply_sampler(&mut self, sampler: &Sampler) {
    self.bind();
    let target = self.raw_texture.target();
    sampler.apply(&self.ctx, target);
    if SET_BIND_NONE_AFTER_WORK {
      self.ctx.bind_texture(target, None);
    }
  }
  pub fn width(&self) -> usize {
    self.raw_texture.desc().width
  }
  pub fn height(&self) -> usize {
    self.raw_texture.desc().height
  }
  pub fn depth(&self) -> usize {
    self.raw_texture.desc().depth
  }
  pub fn target(&self) -> u32 {
    self.raw_texture.target()
  }
  pub fn format(&self) -> PixelFormat {
    self.raw_texture.desc().format
  }
  pub fn channels(&self) -> usize {
    self.raw_texture.channels()
  }
  pub fn raw_texture(&self) -> &RawTexture {
    &self.raw_texture
  }
  fn new_impl<'a>(
    ctx: &ArcGlContext,
    desc: &Texture2dDescriptor,
    write_type: TextureWriteType<'a>,
  ) -> Self {
    Self {
      ctx: ctx.clone(),
      raw_texture: RawTexture::new(ctx, desc, write_type),
    }
  }
}

// ShaderTemplateで生成したmappingを引数に取ってバインドに使う
pub struct TextureMapping<T: TextureMappingAttribute> {
  ctx: ArcGlContext,
  keys: Vec<&'static str>,
  mapping: RwLock<T>,
}
pub trait TextureMappingTrait {
  // returns successed
  fn bind(&self, shader: &Shader);
}
impl<T: TextureMappingAttribute> TextureMapping<T> {
  pub fn new(ctx: &ArcGlContext, mapping: T) -> Self {
    Self {
      ctx: ctx.clone(),
      keys: mapping.keys(),
      mapping: RwLock::new(mapping),
    }
  }
  pub fn write_lock(&self) -> std::sync::RwLockWriteGuard<'_, T> {
    self.mapping.write().unwrap()
  }
  pub fn read_lock(&self) -> std::sync::RwLockReadGuard<'_, T> {
    self.mapping.read().unwrap()
  }
}

impl<T: TextureMappingAttribute> TextureMappingTrait for TextureMapping<T> {
  fn bind(&self, shader: &Shader) {
    let lock = self.mapping.read().unwrap();
    let values = lock.values();
    for i in 0..self.keys.len() {
      if let Some((location, index)) = shader.get_uniform_texture_location(self.keys[i]) {
        match &values[i] {
          ShaderSamplerType::sampler2D(texture) => {
            self.ctx.active_texture(RawTexture::to_slot_enum(*index));
            texture.bind();
            self.ctx.uniform1i(Some(location), *index);
          }
        }
      }
    }
  }
}
