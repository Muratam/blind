use super::*;
// xN のGpuバッファを内部に持つ？
// -> デフォルトは1で後からResizeを設定、くらいの流れで
// -> 差はここで吸収

pub struct IndexBuffer {
  raw_buffer: RawGpuBuffer,
}
impl IndexBuffer {
  pub fn new(gl: &Rc<GlContext>, data: &[IndexBufferType]) -> Self {
    let raw_buffer = RawGpuBuffer::new(gl, data, BufferUsage::Index);
    Self { raw_buffer }
  }
  pub fn raw_buffer(&self) -> &RawGpuBuffer {
    &self.raw_buffer
  }
}

pub struct VertexBuffer {
  raw_buffer: RawGpuBuffer,
}
impl VertexBuffer {
  pub fn new<T>(gl: &Rc<GlContext>, data: &[T]) -> Self {
    let raw_buffer = RawGpuBuffer::new(gl, data, BufferUsage::Vertex);
    Self { raw_buffer }
  }
  pub fn raw_buffer(&self) -> &RawGpuBuffer {
    &self.raw_buffer
  }
}

pub struct UniformBuffer<T: BufferAttribute> {
  gl: Rc<GlContext>,
  name: &'static str,
  raw_buffer: RawGpuBuffer,
  is_dirty: bool,
  data: T,
}
impl<T: BufferAttribute> UniformBuffer<T> {
  pub fn new(gl: &Rc<GlContext>, data: T) -> Self {
    let raw_buffer = RawGpuBuffer::new_untyped(gl, data.ub_data(), BufferUsage::Uniform);
    Self {
      gl: Rc::clone(gl),
      name: data.name(),
      raw_buffer,
      is_dirty: false,
      data,
    }
  }
  pub fn mut_data(&self) -> &mut T {
    self.is_dirty = true;
    &mut self.data
  }
  pub fn data(&self) -> &T {
    &self.data
  }
  // returns successed
  pub fn bind(&mut self, program: &RawShaderProgram) -> bool {
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

pub struct Vao {
  gl: Rc<GlContext>,
  vs_in: VsInTemplate,
  v_buffer: Rc<VertexBuffer>,
  i_buffer: Option<Rc<IndexBuffer>>,
  shader_id_to_raw_vao: std::collections::HashMap<u64, RawVao>,
}
