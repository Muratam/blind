use super::*;
#[derive(PartialEq, Clone, Copy)]
pub enum SamplerMagFilter {
  Linear = gl::LINEAR as isize, // Default
  Nearest = gl::NEAREST as isize,
}
#[derive(PartialEq, Clone, Copy)]
pub enum SamplerMinFilter {
  Linear = gl::LINEAR as isize,
  Nearest = gl::NEAREST as isize,
  LinearMipmapNearest = gl::LINEAR_MIPMAP_NEAREST as isize,
  NearestMipmapLinear = gl::NEAREST_MIPMAP_LINEAR as isize, // default
  LinearMipmapLinear = gl::LINEAR_MIPMAP_LINEAR as isize,
}
#[derive(PartialEq, Clone, Copy)]
pub enum SamplerWrapMode {
  Repeat = gl::REPEAT as isize, // Default
  ClampToEdge = gl::CLAMP_TO_EDGE as isize,
  MirroredRepeat = gl::MIRRORED_REPEAT as isize,
}
#[derive(Clone)]
pub struct Sampler {
  mag_filter: SamplerMagFilter,
  min_filter: SamplerMinFilter,
  wrap_mode_s: SamplerWrapMode,
  wrap_mode_t: SamplerWrapMode,
}

impl Default for Sampler {
  fn default() -> Self {
    Self {
      mag_filter: SamplerMagFilter::Linear,
      min_filter: SamplerMinFilter::NearestMipmapLinear,
      wrap_mode_s: SamplerWrapMode::Repeat,
      wrap_mode_t: SamplerWrapMode::Repeat,
    }
  }
}

impl Sampler {
  pub fn apply(&self, gl: &ArcGlContext, target: u32) {
    gl.tex_parameteri(target, gl::TEXTURE_MAG_FILTER, self.mag_filter as i32);
    gl.tex_parameteri(target, gl::TEXTURE_MIN_FILTER, self.min_filter as i32);
    gl.tex_parameteri(target, gl::TEXTURE_WRAP_S, self.wrap_mode_s as i32);
    gl.tex_parameteri(target, gl::TEXTURE_WRAP_T, self.wrap_mode_t as i32);
  }
}
