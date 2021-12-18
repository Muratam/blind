use super::*;
use prpr::math;
#[allow(non_camel_case_types)]
pub type float = f32;
#[allow(non_camel_case_types)]
pub type vec2 = math::Vec2;
#[allow(non_camel_case_types)]
pub type vec3 = math::Vec3;
#[allow(non_camel_case_types)]
pub type vec4 = math::Vec4;
#[allow(non_camel_case_types)]
pub type uint = u16;
#[allow(non_camel_case_types)]
pub type uvec2 = (uint, uint);
#[allow(non_camel_case_types)]
pub type uvec3 = (uint, uint, uint);
#[allow(non_camel_case_types)]
pub type uvec4 = (uint, uint, uint, uint);
#[allow(non_camel_case_types)]
pub type mat4 = math::Mat4;

#[derive(Debug)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum ShaderPrimitiveType {
  uint(uint),
  uvec2(uvec2),
  uvec3(uvec3),
  uvec4(uvec4),
  float(float),
  vec2(vec2),
  vec3(vec3),
  vec4(vec4),
  mat4(mat4),
}
#[allow(non_camel_case_types)]
#[allow(unused)]
pub enum ShaderSinglePrimitiveType {
  i8 = gl::BYTE as isize,
  i16 = gl::SHORT as isize,
  i32 = gl::INT as isize,
  u8 = gl::UNSIGNED_BYTE as isize,
  u16 = gl::UNSIGNED_SHORT as isize,
  u32 = gl::UNSIGNED_INT as isize,
  f16 = gl::HALF_FLOAT as isize,
  f32 = gl::FLOAT as isize,
}
impl ShaderPrimitiveType {
  pub fn get_single_primitive_type(&self) -> ShaderSinglePrimitiveType {
    type Result = ShaderSinglePrimitiveType;
    match self {
      Self::uint(_) => Result::u16,
      Self::uvec2(_) => Result::u16,
      Self::uvec3(_) => Result::u16,
      Self::uvec4(_) => Result::u16,
      Self::float(_) => Result::f32,
      Self::vec2(_) => Result::f32,
      Self::vec3(_) => Result::f32,
      Self::vec4(_) => Result::f32,
      Self::mat4(_) => Result::f32,
    }
  }
  pub fn get_single_primitive_count(&self) -> i32 {
    match self {
      Self::uint(_) => 1,
      Self::uvec2(_) => 2,
      Self::uvec3(_) => 3,
      Self::uvec4(_) => 4,
      Self::float(_) => 1,
      Self::vec2(_) => 2,
      Self::vec3(_) => 3,
      Self::vec4(_) => 4,
      Self::mat4(_) => 16,
    }
  }
}
pub struct ShaderTemplate {
  vs_code_template: String,
  fs_code_template: String,
  vs_in_template: VsInTemplate,
  pub vs_code_impl: String,
  pub fs_code_impl: String,
}
impl ShaderTemplate {
  pub fn new(
    vs_in_template: VsInTemplate,
    vs_code_template: String,
    fs_code_template: String,
  ) -> Self {
    Self {
      vs_in_template,
      vs_code_template,
      fs_code_template,
      vs_code_impl: String::from(""),
      fs_code_impl: String::from(""),
    }
  }
  pub fn vs_in_template(&self) -> &VsInTemplate {
    &self.vs_in_template
  }
  pub fn vs_code(&self) -> String {
    format!("{}{}", self.vs_code_template, self.vs_code_impl)
  }
  pub fn fs_code(&self) -> String {
    format!("{}{}", self.fs_code_template, self.fs_code_impl)
  }
}

impl ::std::fmt::Display for ShaderTemplate {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    write!(
      f,
      "// vs define\n{}\n// vs impl\n{}\n\n// fs define\n{}\n// fs impl\n{}",
      self.vs_code_template, self.vs_code_impl, self.fs_code_template, self.fs_code_impl
    )
  }
}

pub struct VsInTemplate {
  pub keys: Vec<&'static str>,
  pub values: Vec<ShaderPrimitiveType>,
  pub offsets: Vec<usize>,
  pub size: usize,
}
