use super::*;

#[repr(C)]
pub struct Vec3Attr {
  x: f32,
  y: f32,
  z: f32,
}
impl Vec3Attr {
  pub fn new(v: Vec3) -> Self {
    Self {
      x: v.x,
      y: v.y,
      z: v.z,
    }
  }
}
#[repr(C)]
pub struct Vec4Attr {
  x: f32,
  y: f32,
  z: f32,
  w: f32,
}
impl Vec4Attr {
  pub fn new(v: Vec4) -> Self {
    Self {
      x: v.x,
      y: v.y,
      z: v.z,
      w: v.w,
    }
  }
}

#[derive(Clone, Copy, PartialEq)]
pub enum RawVertexAttributePrimitiveType {
  // i8 = gl::BYTE as isize,
  // i16 = gl::SHORT as isize,
  // u8 = gl::UNSIGNED_BYTE as isize,
  u16 = gl::UNSIGNED_SHORT as isize,
  f32 = gl::FLOAT as isize,
  // f16 = gl::HALF_FLOAT as isize,
}
fn primitive_type_to_byte_size(primitive_type: RawVertexAttributePrimitiveType) -> i32 {
  match primitive_type {
    RawVertexAttributePrimitiveType::u16 => 2,
    RawVertexAttributePrimitiveType::f32 => 4,
  }
}
pub struct RawVertexAttribute {
  pub name: String,
  pub primitive_type: RawVertexAttributePrimitiveType,
  pub count: i32,
  pub location: u32,
}
impl RawVertexAttribute {
  pub fn to_layout_location_str(&self) -> String {
    format!(
      "layout(location = {}) in {} vs_in_{};\n",
      self.location,
      match self.primitive_type {
        RawVertexAttributePrimitiveType::f32 => match self.count {
          1 => "float",
          2 => "vec2",
          3 => "vec3",
          4 => "vec4",
          _ => "vec_over4",
        },
        RawVertexAttributePrimitiveType::u16 => match self.count {
          1 => "uint",
          2 => "uvec2",
          3 => "uvec3",
          4 => "uvec4",
          _ => "uvec_over4",
        },
      },
      self.name
    )
  }
}

pub struct RawVao {
  vao: web_sys::WebGlVertexArrayObject,
}
impl RawVao {
  // attrs の順で全てパッキングされていると仮定
  pub fn new(
    gl: &GlContext,
    attrs: &[RawVertexAttribute],
    v_buffer: &RawGpuBuffer,
    i_buffer: Option<&RawGpuBuffer>,
  ) -> Self {
    let vao = gl.create_vertex_array().expect("failed to create vao");
    gl.bind_vertex_array(Some(&vao));
    if v_buffer.raw_target() != gl::ARRAY_BUFFER {
      log::error("Not Vertex Buffer");
    }
    gl.bind_buffer(gl::ARRAY_BUFFER, Some(v_buffer.raw_buffer()));
    let mut v_type_size: i32 = 0;
    for attr in attrs {
      if attr.count > 4 {
        log::error("attr count > 4");
      }
      v_type_size += primitive_type_to_byte_size(attr.primitive_type) * attr.count as i32;
    }
    let mut offset = 0;
    for attr in attrs {
      gl.enable_vertex_attrib_array(attr.location);
      gl.vertex_attrib_pointer_with_i32(
        attr.location,
        attr.count,
        attr.primitive_type as u32,
        false,
        v_type_size,
        offset,
      );
      offset += primitive_type_to_byte_size(attr.primitive_type) * attr.count as i32;
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
