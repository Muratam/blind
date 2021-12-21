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
// Texture用, 名前だけ欲しい
#[allow(non_camel_case_types)]
pub type sampler2D = Arc<Texture>;

#[derive(Debug, Clone)]
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
#[allow(dead_code)]
pub enum ShaderSamplerType {
  sampler2D(sampler2D),
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
use std::collections::HashMap;
pub type UniformTextureLocation = (web_sys::WebGlUniformLocation, i32);
pub struct ShaderTemplate {
  vs_code_definitions: String,
  fs_code_definitions: String,
  uniform_blocks: Vec<&'static str>,
  uniform_textures: Vec<&'static str>,
  pub vs_code_body: String,
  pub fs_code_body: String,
}
impl ShaderTemplate {
  pub fn new(
    uniform_blocks: Vec<&'static str>,
    uniform_textures: Vec<&'static str>,
    vs_code_definitions: String,
    fs_code_definitions: String,
  ) -> Self {
    Self {
      uniform_blocks,
      uniform_textures,
      vs_code_definitions,
      fs_code_definitions,
      vs_code_body: String::from(""),
      fs_code_body: String::from(""),
    }
  }
  pub fn vs_code(&self) -> String {
    format!("{}{}", self.vs_code_definitions, self.vs_code_body)
  }
  pub fn fs_code(&self) -> String {
    format!("{}{}", self.fs_code_definitions, self.fs_code_body)
  }
  pub fn uniform_blocks(&self) -> &Vec<&'static str> {
    &self.uniform_blocks
  }
  pub fn uniform_textures(&self) -> &Vec<&'static str> {
    &self.uniform_textures
  }
}

impl ::std::fmt::Display for ShaderTemplate {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    write!(
      f,
      "// ------------------\n// vertex shader\n// ------------------\n{}\n{}\n\n// ------------------\n// fragment shader\n// ------------------\n{}\n{}",
      self.vs_code_definitions, self.vs_code_body, self.fs_code_definitions, self.fs_code_body
    )
  }
}

#[derive(Default)]
pub struct VsInTemplate {
  pub keys: Vec<&'static str>,
  pub values: Vec<ShaderPrimitiveType>,
  pub offsets: Vec<usize>,
  pub size: usize,
}

pub trait BufferAttribute {
  fn ub_data(&self) -> &[u8];
  fn name(&self) -> &'static str;
  fn vs_in_template(&self) -> VsInTemplate;
  fn keys(&self) -> Vec<&'static str>;
  fn values(&self) -> Vec<ShaderPrimitiveType>;
  // for dynamic loading
  fn find(&self, key: &str) -> Option<ShaderPrimitiveType>;
  fn from_hashmap(&mut self, map: &HashMap<String, ShaderPrimitiveType>) -> Vec<&'static str>; // returns ignored keys
  fn to_hashmap(&self) -> HashMap<String, ShaderPrimitiveType>;
}

pub trait TextureMappingAttribute {
  fn name(&self) -> &'static str;
  fn keys(&self) -> Vec<&'static str>;
  fn values(&self) -> Vec<ShaderSamplerType>;
  // for dynamic loading
  fn find(&self, key: &str) -> Option<ShaderSamplerType>;
  fn from_hashmap(&mut self, map: &HashMap<String, ShaderSamplerType>) -> Vec<&'static str>; // returns ignored keys
  fn to_hashmap(&self) -> HashMap<String, ShaderSamplerType>;
}
