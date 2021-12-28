use super::*;

// = format
#[derive(Clone, Copy, PartialEq)]
pub enum RawPixelFormatSimple {
  R = gl::RED as isize,
  Rg = gl::RG as isize,
  Rgb = gl::RGB as isize,
  Rgba = gl::RGBA as isize,
  Depth = gl::DEPTH_COMPONENT as isize,
  DepthStencil = gl::DEPTH_STENCIL as isize,
}
impl RawPixelFormatSimple {
  pub fn channels(&self) -> usize {
    match self {
      Self::R => 1,
      Self::Rg => 2,
      Self::Rgb => 3,
      Self::Rgba => 4,
      Self::Depth => 1,
      Self::DepthStencil => 2,
    }
  }
}
// = type
#[derive(Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub enum PixelType {
  u8 = gl::UNSIGNED_BYTE as isize,
  u32 = gl::UNSIGNED_INT as isize,
  f32 = gl::FLOAT as isize,
}
// = internalFormat
#[derive(Clone, Copy, PartialEq)]
pub enum RawPixelFormat {
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
  // depth
  Depth24 = gl::DEPTH_COMPONENT24 as isize,
  Depth32F = gl::DEPTH_COMPONENT32F as isize,
  Depth24Stencil8 = gl::DEPTH24_STENCIL8 as isize,
}
impl RawPixelFormat {
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
      Self::Depth24 => 3,
      Self::Depth32F => 4,
      Self::Depth24Stencil8 => 4,
    }
  }
  pub fn to_simple_format(&self) -> RawPixelFormatSimple {
    match self {
      // R
      Self::R8 => RawPixelFormatSimple::R,
      Self::R8Snorm => RawPixelFormatSimple::R,
      Self::R16F => RawPixelFormatSimple::R,
      Self::R32F => RawPixelFormatSimple::R,
      // RG
      Self::R8G8 => RawPixelFormatSimple::Rg,
      Self::R8G8Snorm => RawPixelFormatSimple::Rg,
      Self::R16G16F => RawPixelFormatSimple::Rg,
      Self::R32G32F => RawPixelFormatSimple::Rg,
      // RGB
      Self::R8G8B8 => RawPixelFormatSimple::Rgb,
      Self::R8G8B8Snorm => RawPixelFormatSimple::Rgb,
      Self::R16G16B16F => RawPixelFormatSimple::Rgb,
      Self::R32G32B32F => RawPixelFormatSimple::Rgb,
      Self::R8G8B8Srgb => RawPixelFormatSimple::Rgb,
      Self::R5G6B5 => RawPixelFormatSimple::Rgb, // non-u
      Self::R11G11B10F => RawPixelFormatSimple::Rgb, // non-u
      // RGBA
      Self::R8G8B8A8 => RawPixelFormatSimple::Rgba,
      Self::R8G8B8A8Snorm => RawPixelFormatSimple::Rgba,
      Self::R16G16B16A16F => RawPixelFormatSimple::Rgba,
      Self::R32G32B32A32F => RawPixelFormatSimple::Rgba,
      Self::R8G8B8A8Srgb => RawPixelFormatSimple::Rgba,
      Self::R4G4B4A4 => RawPixelFormatSimple::Rgba,
      Self::R5G5B5A1 => RawPixelFormatSimple::Rgba, // non-u
      Self::R10G10B10A2 => RawPixelFormatSimple::Rgba, // non-u
      Self::Depth24 => RawPixelFormatSimple::Depth,
      Self::Depth32F => RawPixelFormatSimple::Depth,
      Self::Depth24Stencil8 => RawPixelFormatSimple::DepthStencil,
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
      Self::R8Snorm => PixelType::u8,         // not specified
      Self::R8G8Snorm => PixelType::u8,       // not specified
      Self::R8G8B8Snorm => PixelType::u8,     // not specified
      Self::R8G8B8A8Snorm => PixelType::u8,   // not specified
      Self::Depth24 => PixelType::u32,        // not specified
      Self::Depth32F => PixelType::f32,       // not specified
      Self::Depth24Stencil8 => PixelType::u8, // not specified
    }
  }
}

#[derive(Clone)]
pub struct RawTexture2dDescriptor {
  pub width: usize,
  pub height: usize,
  pub format: RawPixelFormat,
  pub mipmap: bool,
}
#[derive(Clone)]
pub struct RawTextureDescriptor {
  pub format: RawPixelFormat,
  pub width: usize,
  pub height: usize,
  pub depth: usize,
  pub mipmap: bool,
  pub target: u32,
}
impl RawTextureDescriptor {
  pub fn from_2d_descriptor(desc: &RawTexture2dDescriptor) -> Self {
    Self {
      format: desc.format,
      width: desc.width,
      height: desc.height,
      depth: 1,
      mipmap: desc.mipmap,
      target: gl::TEXTURE_2D,
    }
  }
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

use std::sync::atomic::{AtomicUsize, Ordering};
static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
pub struct RawTexture {
  raw_texture: web_sys::WebGlTexture,
  desc: RawTextureDescriptor,
  texture_id: u64,
}
impl RawTexture {
  // pub fn new_cubemap() { target = TEXTURE_CUBE_MAP_??; }
  pub fn new<'a>(desc: &RawTexture2dDescriptor, write_type: TextureWriteType<'a>) -> Self {
    let ctx = Instance::ctx();
    let raw_texture = ctx.create_texture().expect("failed to create texture");
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
    ctx.bind_texture(target, Some(&raw_texture));
    if pixels.is_some() || write_type == TextureWriteType::Uninitialized {
      ctx
        .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
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
      ctx.generate_mipmap(target);
    }
    if SET_BIND_NONE_AFTER_WORK {
      ctx.bind_texture(target, None);
    }
    Self {
      raw_texture,
      texture_id: ID_COUNTER.fetch_add(1, Ordering::SeqCst) as u64,
      desc: RawTextureDescriptor::from_2d_descriptor(desc),
    }
  }
  pub fn write(&self) {
    log::error("not implemented(RawTexture::write)");
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
  pub fn depth(&self) -> usize {
    self.desc.depth
  }
  pub fn format(&self) -> RawPixelFormat {
    self.desc.format
  }
  pub fn desc(&self) -> &RawTextureDescriptor {
    &self.desc
  }
  pub fn target(&self) -> u32 {
    self.desc.target
  }
  pub fn bind(&self) {
    let ctx = Instance::ctx();
    let target = self.target();
    ctx.bind_texture(target, Some(&self.raw_texture));
  }

  pub fn channels(&self) -> usize {
    self.desc.format.to_simple_format().channels()
  }
  pub fn raw_texture(&self) -> &web_sys::WebGlTexture {
    &self.raw_texture
  }
  pub fn texture_id(&self) -> u64 {
    self.texture_id
  }
  pub fn to_slot_enum(i: i32) -> u32 {
    match i {
      0 => gl::TEXTURE0,
      1 => gl::TEXTURE1,
      2 => gl::TEXTURE2,
      3 => gl::TEXTURE3,
      4 => gl::TEXTURE4,
      5 => gl::TEXTURE5,
      6 => gl::TEXTURE6,
      7 => gl::TEXTURE7,
      8 => gl::TEXTURE8,
      9 => gl::TEXTURE9,
      10 => gl::TEXTURE10,
      11 => gl::TEXTURE11,
      12 => gl::TEXTURE12,
      13 => gl::TEXTURE13,
      14 => gl::TEXTURE14,
      15 => gl::TEXTURE15,
      16 => gl::TEXTURE16,
      17 => gl::TEXTURE17,
      18 => gl::TEXTURE18,
      19 => gl::TEXTURE19,
      20 => gl::TEXTURE20,
      21 => gl::TEXTURE21,
      22 => gl::TEXTURE22,
      23 => gl::TEXTURE23,
      24 => gl::TEXTURE24,
      25 => gl::TEXTURE25,
      26 => gl::TEXTURE26,
      27 => gl::TEXTURE27,
      28 => gl::TEXTURE28,
      29 => gl::TEXTURE29,
      30 => gl::TEXTURE30,
      31 => gl::TEXTURE31,
      _ => {
        log::error(format!("TEXTURE{} over the range", i));
        0
      }
    }
  }
}
impl Drop for RawTexture {
  fn drop(&mut self) {
    let ctx = Instance::ctx();
    ctx.delete_texture(Some(&self.raw_texture));
  }
}
