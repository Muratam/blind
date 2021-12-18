use super::*;
// xN のGpuバッファを内部に持つ？
// -> デフォルトは1で後からResizeを設定、くらいの流れで
// -> 差はここで吸収

pub struct IndexBuffer {
  raw_buffer: RawGpuBuffer,
  data: Vec<IndexBufferType>,
}
impl IndexBuffer {
  pub fn new(gl: &Rc<GlContext>, data: Vec<IndexBufferType>) -> Self {
    Self {
      raw_buffer: RawGpuBuffer::new(gl, data.as_slice(), BufferUsage::Index),
      data,
    }
  }
  pub fn raw_buffer(&self) -> &RawGpuBuffer {
    &self.raw_buffer
  }
}

pub struct VertexBuffer<T: BufferAttribute> {
  raw_buffer: RawGpuBuffer,
  template: VsInTemplate,
  data: Vec<T>,
}

impl<T: BufferAttribute> VertexBuffer<T> {
  pub fn new(gl: &Rc<GlContext>, data: Vec<T>) -> Self {
    let template = if data.len() > 0 {
      data[0].vs_in_template()
    } else {
      Default::default()
    };
    Self {
      raw_buffer: RawGpuBuffer::new(gl, data.as_slice(), BufferUsage::Vertex),
      template,
      data,
    }
  }
  pub fn raw_buffer(&self) -> &RawGpuBuffer {
    &self.raw_buffer
  }
  pub fn template(&self) -> &VsInTemplate {
    &self.template
  }
}

pub struct UniformBuffer<T: BufferAttribute> {
  gl: Rc<GlContext>,
  raw_buffer: RawGpuBuffer,
  data: T,
  name: &'static str,
  is_dirty: bool,
}
pub trait UniformBufferTrait {
  // returns successed
  fn bind(&mut self, program: &RawShaderProgram) -> bool;
}
impl<T: BufferAttribute> UniformBuffer<T> {
  pub fn new(gl: &Rc<GlContext>, data: T) -> Self {
    Self {
      gl: Rc::clone(gl),
      name: data.name(),
      raw_buffer: RawGpuBuffer::new_untyped(gl, data.ub_data(), BufferUsage::Uniform),
      is_dirty: false,
      data,
    }
  }
  pub fn mut_data(&mut self) -> &mut T {
    self.is_dirty = true;
    &mut self.data
  }
  pub fn data(&self) -> &T {
    &self.data
  }
}
impl<T: BufferAttribute> UniformBufferTrait for UniformBuffer<T> {
  fn bind(&mut self, program: &RawShaderProgram) -> bool {
    if self.is_dirty {
      self.raw_buffer.write_untyped(0, self.data.ub_data());
      self.is_dirty = false;
    }
    let u_index = self
      .gl
      .get_uniform_block_index(&program.raw_program(), self.name);
    if u_index == gl::INVALID_INDEX {
      return false;
    }
    self.gl.bind_buffer_base(
      gl::UNIFORM_BUFFER,
      u_index,
      Some(self.raw_buffer.raw_buffer()),
    );
    return true;
  }
}
use std::collections::HashMap;
pub struct Vao<T: BufferAttribute> {
  gl: Rc<GlContext>,
  v_buffer: VertexBuffer<T>,
  i_buffer: Option<IndexBuffer>,
  shader_id_to_raw_vao: HashMap<u64, RawVao>,
}
pub trait VaoTrait {
  // returns successed
  fn bind(&mut self, program: &RawShaderProgram);
}
impl<T: BufferAttribute> Vao<T> {
  pub fn new(gl: &Rc<GlContext>, v_buffer: VertexBuffer<T>, i_buffer: IndexBuffer) -> Self {
    Self {
      gl: Rc::clone(gl),
      v_buffer,
      i_buffer: Some(i_buffer),
      shader_id_to_raw_vao: HashMap::new(),
    }
  }
  pub fn new_without_index_buffer(gl: &Rc<GlContext>, v_buffer: VertexBuffer<T>) -> Self {
    Self {
      gl: Rc::clone(gl),
      v_buffer,
      i_buffer: None,
      shader_id_to_raw_vao: HashMap::new(),
    }
  }
}
impl<T: BufferAttribute> VaoTrait for Vao<T> {
  fn bind(&mut self, program: &RawShaderProgram) {
    let id = program.raw_program_id();
    if let Some(raw_vao) = self.shader_id_to_raw_vao.get(&id) {
      self.gl.bind_vertex_array(Some(raw_vao.get_raw_vao()));
    }
    let i_buffer = if let Some(i_buffer) = &self.i_buffer {
      Some(i_buffer.raw_buffer())
    } else {
      None
    };
    let raw_vao = RawVao::new(
      &self.gl,
      program.raw_program(),
      Some((self.v_buffer.template(), self.v_buffer.raw_buffer())),
      i_buffer,
    );
    self.gl.bind_vertex_array(Some(raw_vao.get_raw_vao()));
    self.shader_id_to_raw_vao.insert(id, raw_vao);
  }
}
