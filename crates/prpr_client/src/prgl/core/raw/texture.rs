use super::*;
// = format
#[derive(Clone, Copy, PartialEq)]
pub enum PixelFormatSimple {
  R = gl::RED as isize,
  Rg = gl::RG as isize,
  Rgb = gl::RGB as isize,
  Rgba = gl::RGBA as isize,
}
impl PixelFormatSimple {
  pub fn channels(&self) -> usize {
    match self {
      Self::R => 1,
      Self::Rg => 2,
      Self::Rgb => 3,
      Self::Rgba => 4,
    }
  }
}
// = type
#[derive(Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub enum PixelType {
  u8 = gl::UNSIGNED_BYTE as isize,
  f32 = gl::FLOAT as isize,
}
// = internalFormat
#[derive(Clone, Copy, PartialEq)]
pub enum PixelFormat {
  // color renderble & texture filterable
  R8 = gl::R8 as isize,
  R8G8 = gl::RG8 as isize,
  R8G8B8 = gl::RGB8 as isize,
  R8G8B8A8 = gl::RGBA8 as isize,
  R8G8B8A8Srgb = gl::SRGB8_ALPHA8 as isize,
  R4G4B4A4 = gl::RGBA4 as isize,
  R5G6B5 = gl::RGB565 as isize,
  R5G5B5A1 = gl::RGB5_A1 as isize,
  R10G10B10A2 = gl::RGB10_A2 as isize,
  // only texture filterable
  R8Snorm = gl::R8_SNORM as isize,
  R8G8Snorm = gl::RG8_SNORM as isize,
  R8G8B8Snorm = gl::RGB8_SNORM as isize,
  R8G8B8A8Snorm = gl::RGBA8_SNORM as isize,
  R8G8B8Srgb = gl::SRGB8 as isize,
  R16F = gl::R16F as isize,
  R16G16F = gl::RG16F as isize,
  R16G16B16F = gl::RGB16F as isize,
  R16G16B16A16F = gl::RGBA16F as isize,
  R11G11B10F = gl::R11F_G11F_B10F as isize,
  // none
  R32F = gl::R32F as isize,
  R32G32F = gl::RG32F as isize,
  R32G32B32F = gl::RGB32F as isize,
  R32G32B32A32F = gl::RGBA32F as isize,
}
impl PixelFormat {
  // bit per pixel
  pub fn bpp(&self) -> usize {
    match self {
      // R
      Self::R8 => 1,
      Self::R8Snorm => 1,
      Self::R16F => 2,
      Self::R32F => 4,
      // RG
      Self::R8G8 => 2,
      Self::R8G8Snorm => 2,
      Self::R16G16F => 4,
      Self::R32G32F => 8,
      // RGB
      Self::R8G8B8 => 3,
      Self::R8G8B8Snorm => 3,
      Self::R8G8B8Srgb => 3,
      Self::R16G16B16F => 6,
      Self::R32G32B32F => 12,
      Self::R5G6B5 => 2,
      Self::R11G11B10F => 4,
      // RGBA
      Self::R8G8B8A8 => 4,
      Self::R8G8B8A8Snorm => 4,
      Self::R8G8B8A8Srgb => 4,
      Self::R16G16B16A16F => 8,
      Self::R32G32B32A32F => 16,
      Self::R4G4B4A4 => 2,
      Self::R5G5B5A1 => 2,
      Self::R10G10B10A2 => 4,
    }
  }
  pub fn to_simple_format(&self) -> PixelFormatSimple {
    match self {
      // R
      Self::R8 => PixelFormatSimple::R,
      Self::R8Snorm => PixelFormatSimple::R,
      Self::R16F => PixelFormatSimple::R,
      Self::R32F => PixelFormatSimple::R,
      // RG
      Self::R8G8 => PixelFormatSimple::Rg,
      Self::R8G8Snorm => PixelFormatSimple::Rg,
      Self::R16G16F => PixelFormatSimple::Rg,
      Self::R32G32F => PixelFormatSimple::Rg,
      // RGB
      Self::R8G8B8 => PixelFormatSimple::Rgb,
      Self::R8G8B8Snorm => PixelFormatSimple::Rgb,
      Self::R16G16B16F => PixelFormatSimple::Rgb,
      Self::R32G32B32F => PixelFormatSimple::Rgb,
      Self::R8G8B8Srgb => PixelFormatSimple::Rgb,
      Self::R5G6B5 => PixelFormatSimple::Rgb,     // non-u
      Self::R11G11B10F => PixelFormatSimple::Rgb, // non-u
      // RGBA
      Self::R8G8B8A8 => PixelFormatSimple::Rgba,
      Self::R8G8B8A8Snorm => PixelFormatSimple::Rgba,
      Self::R16G16B16A16F => PixelFormatSimple::Rgba,
      Self::R32G32B32A32F => PixelFormatSimple::Rgba,
      Self::R8G8B8A8Srgb => PixelFormatSimple::Rgba,
      Self::R4G4B4A4 => PixelFormatSimple::Rgba,
      Self::R5G5B5A1 => PixelFormatSimple::Rgba, // non-u
      Self::R10G10B10A2 => PixelFormatSimple::Rgba, // non-u
    }
  }
  pub fn to_writable_uniform_type(&self) -> PixelType {
    // https://www.khronos.org/registry/webgl/specs/latest/2.0/#TEXTURE_TYPES_FORMATS_FROM_DOM_ELEMENTS_TABLE
    match self {
      // u8
      Self::R8 => PixelType::u8,
      Self::R8G8 => PixelType::u8,
      Self::R8G8B8 => PixelType::u8,
      Self::R8G8B8A8 => PixelType::u8,
      Self::R8G8B8Srgb => PixelType::u8,
      Self::R8G8B8A8Srgb => PixelType::u8,
      // f32
      Self::R16F => PixelType::f32,          // may HALF_FLOAT
      Self::R16G16F => PixelType::f32,       // may HALF_FLOAT
      Self::R16G16B16F => PixelType::f32,    // may HALF_FLOAT
      Self::R16G16B16A16F => PixelType::f32, // may HALF_FLOAT
      Self::R32F => PixelType::f32,
      Self::R32G32F => PixelType::f32,
      Self::R32G32B32F => PixelType::f32,
      Self::R32G32B32A32F => PixelType::f32,
      // u8(-)
      Self::R5G6B5 => PixelType::u8,      // may UNSIGNED_SHORT_5_6_5
      Self::R4G4B4A4 => PixelType::u8,    // may UNSIGNED_SHORT_4_4_4_4
      Self::R5G5B5A1 => PixelType::u8,    // may UNSIGNED_SHORT_5_5_5_1
      Self::R10G10B10A2 => PixelType::u8, // only UNSIGNED_INT_2_10_10_10_REV
      // f32(-)
      Self::R11G11B10F => PixelType::f32, // may UNSIGNED_INT_10F_11F_11F_REV
      // u8 ?
      Self::R8Snorm => PixelType::u8,       // not specified
      Self::R8G8Snorm => PixelType::u8,     // not specified
      Self::R8G8B8Snorm => PixelType::u8,   // not specified
      Self::R8G8B8A8Snorm => PixelType::u8, // not specified
    }
  }
}

#[derive(Clone)]
pub struct TextureDescriptor {
  format: PixelFormat,
  width: usize,
  height: usize,
  mipmap: bool,
}
pub struct RawTexture {
  gl: Rc<GlContext>,
  raw_texture: web_sys::WebGlTexture,
  desc: TextureDescriptor,
  target: u32,
}
#[derive(PartialEq)]
#[allow(non_camel_case_types)]
pub enum TextureWriteType<'a> {
  Uninitialized,
  Zero,
  One,
  u8(&'a [u8]),
  f32(&'a [f32]),
  ImageBitmap(&'a web_sys::ImageBitmap),
  ImageData(&'a web_sys::ImageData),
  HtmlImageElement(&'a web_sys::HtmlImageElement),
  HtmlCanvasElement(&'a web_sys::HtmlCanvasElement),
  HtmlVideoElement(&'a web_sys::HtmlVideoElement),
}
impl RawTexture {
  // pub fn new_cubemap() { target = TEXTURE_CUBE_MAP_??; }
  pub fn new<'a>(
    gl: &Rc<GlContext>,
    desc: &TextureDescriptor,
    write_type: TextureWriteType<'a>,
  ) -> Self {
    let raw_texture = gl.create_texture().expect("failed to create texture");
    let target = gl::TEXTURE_2D;
    let level = 0;
    let internalformat = desc.format as i32;
    let width = desc.width;
    let height = desc.height;
    let border = 0;
    let format = desc.format.to_simple_format();
    let type_ = desc.format.to_writable_uniform_type();
    let bpp = desc.format.bpp();
    let u8_array_size = bpp * width * height;
    let tmpvec = match write_type {
      TextureWriteType::Zero => vec![0x00; u8_array_size],
      TextureWriteType::One => vec![0xff; u8_array_size],
      _ => Vec::new(),
    };
    let pixels: Option<&[u8]> = match write_type {
      TextureWriteType::Uninitialized => None,
      TextureWriteType::Zero => Some(tmpvec.as_slice()),
      TextureWriteType::One => Some(tmpvec.as_slice()),
      TextureWriteType::u8(pixels) => Some(pixels),
      TextureWriteType::f32(f_pixels) => {
        let u8_size = 4 * f_pixels.len();
        let ptr = f_pixels.as_ptr() as *const u8;
        let u8_data: &[u8] = unsafe { ::core::slice::from_raw_parts(ptr, u8_size) };
        Some(u8_data)
      }
      _ => None,
    };
    gl.bind_texture(target, Some(&raw_texture));
    if pixels.is_some() || write_type == TextureWriteType::Uninitialized {
      gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        target,
        level,
        internalformat,
        width as i32,
        height as i32,
        border,
        format as u32,
        type_ as u32,
        pixels,
      )
      .ok();
    } else {
      log::error("currently not supported texture type specified.");
    }
    if desc.mipmap {
      gl.generate_mipmap(target);
    }
    if SET_BIND_NONE_AFTER_WORK {
      gl.bind_texture(target, None);
    }
    Self {
      gl: Rc::clone(gl),
      raw_texture,
      desc: desc.clone(),
      target,
    }
  }
  pub fn write(&self) {
    // TODO:
    // tex_sub_image_2d_with_u32_and_u32_and_image(
    // copy_tex_sub_image_2d
    // copy_tex_image_2d
    // for compressed format:
    //  compressed_tex_image_2d_with_u8_array
    //  compressed_tex_sub_image_2d_with_array_buffer_view
  }
  pub fn width(&self) -> usize {
    self.desc.width
  }
  pub fn height(&self) -> usize {
    self.desc.height
  }
  pub fn format(&self) -> PixelFormat {
    self.desc.format
  }
  pub fn target(&self) -> u32 {
    self.target
  }
  pub fn channels(&self) -> usize {
    self.desc.format.to_simple_format().channels()
  }
  pub fn raw_texture(&self) -> &web_sys::WebGlTexture {
    &self.raw_texture
  }
}
impl Drop for RawTexture {
  fn drop(&mut self) {
    self.gl.delete_texture(Some(&self.raw_texture));
  }
}
