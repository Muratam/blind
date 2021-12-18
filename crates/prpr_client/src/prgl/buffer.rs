use super::*;
// xN のGpuバッファを内部に持つ？
// -> デフォルトは1で後からResizeを設定、くらいの流れで
// -> 差はここで吸収

pub struct IndexBuffer {
  gl: Rc<GlContext>,
  raw_buffer: RawGpuBuffer,
}
impl IndexBuffer {
  pub fn new(gl: &Rc<GlContext>, data: &[IndexBufferType]) -> Self {
    let raw_buffer = RawGpuBuffer::new(gl, data, BufferUsage::Index);
    Self {
      gl: Rc::clone(gl),
      raw_buffer,
    }
  }
  pub fn raw_buffer(&self) -> &RawGpuBuffer {
    &self.raw_buffer
  }
}

pub struct VertexBuffer {
  gl: Rc<GlContext>,
  raw_buffer: RawGpuBuffer,
}
impl VertexBuffer {
  pub fn new<T>(gl: &Rc<GlContext>, data: &[T]) -> Self {
    let raw_buffer = RawGpuBuffer::new(gl, data, BufferUsage::Vertex);
    Self {
      gl: Rc::clone(gl),
      raw_buffer,
    }
  }
  pub fn raw_buffer(&self) -> &RawGpuBuffer {
    &self.raw_buffer
  }
}

pub struct UniformBuffer {
  gl: Rc<GlContext>,
  raw_buffer: RawGpuBuffer,
}
impl UniformBuffer {
  pub fn new<T: BufferAttribute>(gl: &Rc<GlContext>, data: &T) -> Self {
    let raw_buffer = RawGpuBuffer::new(gl, data.ub_data(), BufferUsage::Uniform);
    Self {
      gl: Rc::clone(gl),
      raw_buffer,
    }
  }
  pub fn raw_buffer(&self) -> &RawGpuBuffer {
    &self.raw_buffer
  }
}

pub struct Vao {
  gl: Rc<GlContext>,
  vs_in: VsInTemplate,
  v_buffer: Rc<VertexBuffer>,
  i_buffer: Option<Rc<IndexBuffer>>,
  shader_id_to_raw_vao: std::collections::HashMap<u64, RawVao>,
}
