use super::*;

pub struct Texture {
  raw_texture: RawTexture,
}
pub type Texture2dDescriptor = RawTexture2dDescriptor;
pub type PixelFormat = RawPixelFormat;
impl Texture {
  pub fn new_rgba_map<F: Fn(f32, f32) -> Vec4>(width: usize, height: usize, color_fn: F) -> Self {
    let size = width * height * 4;
    let mut data: Vec<u8> = vec![0; size];
    fn clamp(x: f32) -> u8 {
      if x > 1.0 {
        255
      } else if x < 0.0 {
        0
      } else {
        (x * 255.0) as u8
      }
    }
    {
      let mut i = 0;
      let inv_width = 1.0 / width as f32;
      let inv_height = 1.0 / height as f32;
      for y in 0..height {
        for x in 0..width {
          let color = color_fn(x as f32 * inv_width, 1.0 - (y as f32 * inv_height));
          data[i] = clamp(color.x);
          data[i + 1] = clamp(color.y);
          data[i + 2] = clamp(color.z);
          data[i + 3] = clamp(color.w);
          i += 4;
        }
      }
    }
    Self::new_bytes(
      &Texture2dDescriptor {
        width,
        height,
        format: PixelFormat::R8G8B8A8,
        mipmap: true,
      },
      data.as_slice(),
    )
  }
  pub fn new_bytes(desc: &Texture2dDescriptor, data: &[u8]) -> Self {
    Self::new_impl(desc, TextureWriteType::u8(data))
  }
  pub fn new_floats(desc: &Texture2dDescriptor, data: &[f32]) -> Self {
    Self::new_impl(desc, TextureWriteType::f32(data))
  }
  pub fn new_uninitialized(desc: &Texture2dDescriptor) -> Self {
    Self::new_impl(desc, TextureWriteType::Uninitialized)
  }
  pub fn new_fill_zero(desc: &Texture2dDescriptor) -> Self {
    Self::new_impl(desc, TextureWriteType::Zero)
  }
  pub fn new_fill_one(desc: &Texture2dDescriptor) -> Self {
    Self::new_impl(desc, TextureWriteType::One)
  }
  pub fn new_image_bitmap(desc: &Texture2dDescriptor, data: &web_sys::ImageBitmap) -> Self {
    Self::new_impl(desc, TextureWriteType::ImageBitmap(data))
  }
  pub fn new_image_data(desc: &Texture2dDescriptor, data: &web_sys::ImageData) -> Self {
    Self::new_impl(desc, TextureWriteType::ImageData(data))
  }
  pub fn new_html_image_element(
    desc: &Texture2dDescriptor,
    data: &web_sys::HtmlImageElement,
  ) -> Self {
    Self::new_impl(desc, TextureWriteType::HtmlImageElement(data))
  }
  pub fn new_html_canvas_element(
    desc: &Texture2dDescriptor,
    data: &web_sys::HtmlCanvasElement,
  ) -> Self {
    Self::new_impl(desc, TextureWriteType::HtmlCanvasElement(data))
  }
  pub fn new_html_video_element(
    desc: &Texture2dDescriptor,
    data: &web_sys::HtmlVideoElement,
  ) -> Self {
    Self::new_impl(desc, TextureWriteType::HtmlVideoElement(data))
  }
  pub fn apply_sampler(&mut self, sampler: &Sampler) {
    self.raw_texture().bind();
    let target = self.raw_texture.target();
    sampler.apply(target);
    if SET_BIND_NONE_AFTER_WORK {
      let ctx = Instance::ctx();
      ctx.bind_texture(target, None);
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
  fn new_impl<'a>(desc: &Texture2dDescriptor, write_type: TextureWriteType<'a>) -> Self {
    Self {
      raw_texture: RawTexture::new(desc, write_type),
    }
  }
}

// ShaderTemplateで生成したmappingを引数に取ってバインドに使う
pub struct TextureMapping<T: TextureMappingAttribute> {
  keys: Vec<&'static str>,
  mapping: T,
}
pub trait TextureMappingTrait {
  fn bind(&self, cmd: &mut Command);
}
impl<T: TextureMappingAttribute> TextureMapping<T> {
  pub fn new(mapping: T) -> Self {
    Self {
      keys: mapping.keys(),
      mapping: mapping,
    }
  }
}

impl<T: TextureMappingAttribute> TextureMappingTrait for TextureMapping<T> {
  fn bind(&self, cmd: &mut Command) {
    if let Some(shader) = cmd.current_shader() {
      let shader = shader.clone();
      let values = self.mapping.values();
      for i in 0..self.keys.len() {
        if let Some(utl) = shader.uniform_texture_location(self.keys[i]) {
          match &values[i] {
            ShaderSamplerType::sampler2D(texture) => {
              cmd.set_uniform_texture(texture.read().raw_texture(), &utl);
            }
          }
        }
      }
    }
  }
}
impl<T: TextureMappingAttribute> TextureMappingTrait for Reader<TextureMapping<T>> {
  fn bind(&self, cmd: &mut Command) {
    self.read().bind(cmd);
  }
}

impl<T: TextureMappingAttribute + 'static> PipelineBindable for Reader<TextureMapping<T>> {
  fn bind_pipeline(&self, pipeline: &mut Pipeline) {
    pipeline.add_texture_mapping(&self);
  }
}
