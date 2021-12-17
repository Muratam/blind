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
    match self {
      ShaderPrimitiveType::uint(_) => ShaderSinglePrimitiveType::u16,
      ShaderPrimitiveType::uvec2(_) => ShaderSinglePrimitiveType::u16,
      ShaderPrimitiveType::uvec3(_) => ShaderSinglePrimitiveType::u16,
      ShaderPrimitiveType::uvec4(_) => ShaderSinglePrimitiveType::u16,
      ShaderPrimitiveType::float(_) => ShaderSinglePrimitiveType::f32,
      ShaderPrimitiveType::vec2(_) => ShaderSinglePrimitiveType::f32,
      ShaderPrimitiveType::vec3(_) => ShaderSinglePrimitiveType::f32,
      ShaderPrimitiveType::vec4(_) => ShaderSinglePrimitiveType::f32,
      ShaderPrimitiveType::mat4(_) => ShaderSinglePrimitiveType::f32,
    }
  }
  pub fn get_single_primitive_count(&self) -> i32 {
    match self {
      ShaderPrimitiveType::uint(_) => 1,
      ShaderPrimitiveType::uvec2(_) => 2,
      ShaderPrimitiveType::uvec3(_) => 3,
      ShaderPrimitiveType::uvec4(_) => 4,
      ShaderPrimitiveType::float(_) => 1,
      ShaderPrimitiveType::vec2(_) => 2,
      ShaderPrimitiveType::vec3(_) => 3,
      ShaderPrimitiveType::vec4(_) => 4,
      ShaderPrimitiveType::mat4(_) => 16,
    }
  }
}
impl ShaderSinglePrimitiveType {
  pub fn get_byte_size(&self) -> i32 {
    match self {
      ShaderSinglePrimitiveType::i8 => 1,
      ShaderSinglePrimitiveType::i16 => 2,
      ShaderSinglePrimitiveType::i32 => 4,
      ShaderSinglePrimitiveType::u8 => 1,
      ShaderSinglePrimitiveType::u16 => 2,
      ShaderSinglePrimitiveType::u32 => 4,
      ShaderSinglePrimitiveType::f16 => 2,
      ShaderSinglePrimitiveType::f32 => 4,
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

pub fn shader_vb_hash_location(key: &str) -> u64 {
  use std::hash::{Hash, Hasher};
  let mut hasher = std::collections::hash_map::DefaultHasher::new();
  key.hash(&mut hasher);
  // gl::MAX_VERTEX_ATTRIBS; がめっちゃちいさい
  hasher.finish() & 0xfff
}

pub struct RawVao {
  vao: web_sys::WebGlVertexArrayObject,
}
impl RawVao {
  pub fn new(
    gl: &GlContext,
    program: &web_sys::WebGlProgram,
    vs_in: &VsInTemplate,
    v_buffer: &RawGpuBuffer,
    i_buffer: Option<&RawGpuBuffer>,
  ) -> Self {
    let vao = gl.create_vertex_array().expect("failed to create vao");
    gl.bind_vertex_array(Some(&vao));
    if v_buffer.raw_target() != gl::ARRAY_BUFFER {
      log::error("Not Vertex Buffer");
    }
    gl.bind_buffer(gl::ARRAY_BUFFER, Some(v_buffer.raw_buffer()));
    assert_eq!(vs_in.offsets.len(), vs_in.keys.len());
    assert_eq!(vs_in.values.len(), vs_in.keys.len());
    for i in 0..vs_in.offsets.len() {
      let location = gl.get_attrib_location(program, vs_in.keys[i]);
      if location < 0 {
        log::error(format!("no vertex attribute: {}", vs_in.keys[i]));
        continue;
      }
      gl.enable_vertex_attrib_array(location as u32);
      let value = &vs_in.values[i];
      gl.vertex_attrib_pointer_with_i32(
        location as u32,
        value.get_single_primitive_count(),
        value.get_single_primitive_type() as u32,
        false,
        vs_in.size as i32,
        vs_in.offsets[i] as i32,
      );
    }
    if let Some(i_buffer) = i_buffer {
      if i_buffer.raw_target() != gl::ELEMENT_ARRAY_BUFFER {
        log::error("Not Index Buffer");
      }
      gl.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(i_buffer.raw_buffer()));
    }
    if SET_BIND_NONE_AFTER_WORK {
      gl.bind_vertex_array(None);
      gl.bind_buffer(gl::ARRAY_BUFFER, None);
      if i_buffer.is_some() {
        gl.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, None);
      }
    }
    Self { vao }
  }

  pub fn get_raw_vao(&self) -> &web_sys::WebGlVertexArrayObject {
    &self.vao
  }
}
