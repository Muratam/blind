use super::*;
// xN のGpuバッファを内部に持つ？
// -> デフォルトは1で後からResizeを設定、くらいの流れで
// -> 差はここで吸収

pub struct IndexBuffer {
  raw_buffer: RawBuffer,
  data: Vec<IndexBufferType>,
}
impl IndexBuffer {
  pub fn new(gl: &Arc<GlContext>, data: Vec<IndexBufferType>) -> Self {
    Self {
      raw_buffer: RawBuffer::new(gl, data.as_slice(), BufferUsage::Index),
      data,
    }
  }
  pub fn raw_buffer(&self) -> &RawBuffer {
    &self.raw_buffer
  }
  pub fn len(&self) -> usize {
    self.data.len()
  }
}

pub struct VertexBuffer<T: BufferAttribute> {
  raw_buffer: RawBuffer,
  template: VsInTemplate,
  data: Vec<T>,
}

impl<T: BufferAttribute> VertexBuffer<T> {
  pub fn new(gl: &Arc<GlContext>, data: Vec<T>) -> Self {
    let template = if data.len() > 0 {
      data[0].vs_in_template()
    } else {
      Default::default()
    };
    Self {
      raw_buffer: RawBuffer::new(gl, data.as_slice(), BufferUsage::Vertex),
      template,
      data,
    }
  }
  pub fn raw_buffer(&self) -> &RawBuffer {
    &self.raw_buffer
  }
  pub fn template(&self) -> &VsInTemplate {
    &self.template
  }
  pub fn len(&self) -> usize {
    self.data.len()
  }
}

pub struct UniformBuffer<T: BufferAttribute> {
  gl: Arc<GlContext>,
  raw_buffer: RawBuffer,
  data: T,
  name: &'static str,
  is_dirty: bool,
}
pub trait UniformBufferTrait {
  // returns successed
  fn bind(&mut self, program: &RawShaderProgram) -> bool;
}
// 裏でも更新する
pub type UniformBufferDynPtr = Arc<RwLock<dyn UniformBufferTrait>>;
pub type UniformBufferPtr<T> = Arc<RwLock<UniformBuffer<T>>>;
impl<T: BufferAttribute> UniformBuffer<T> {
  pub fn new(gl: &Arc<GlContext>, data: T) -> Self {
    Self {
      gl: Arc::clone(gl),
      name: data.name(),
      raw_buffer: RawBuffer::new_untyped(gl, data.ub_data(), BufferUsage::Uniform),
      is_dirty: false,
      data,
    }
  }
  pub fn data_mut(&mut self) -> &mut T {
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
