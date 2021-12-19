use super::*;
// xN のGpuバッファを内部に持つ？
// -> デフォルトは1で後からResizeを設定、くらいの流れで
// -> 差はここで吸収

pub struct IndexBuffer {
  raw_buffer: RawBuffer,
  data: Vec<IndexBufferType>,
}
impl IndexBuffer {
  pub fn new(gl: &ArcGlContext, data: Vec<IndexBufferType>) -> Self {
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
  pub fn new(gl: &ArcGlContext, data: Vec<T>) -> Self {
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
  gl: ArcGlContext,
  raw_buffer: RawBuffer,
  data: RwLock<T>,
  name: &'static str,
  is_dirty: Mutex<bool>,
}
pub trait UniformBufferTrait {
  // returns successed
  fn bind(&self, program: &RawShaderProgram) -> bool;
}
impl<T: BufferAttribute> UniformBuffer<T> {
  pub fn new(gl: &ArcGlContext, data: T) -> Self {
    Self {
      gl: gl.clone(),
      name: data.name(),
      raw_buffer: RawBuffer::new_untyped(gl, data.ub_data(), BufferUsage::Uniform),
      is_dirty: Mutex::new(false),
      data: RwLock::new(data),
    }
  }
  pub fn write(&self) -> std::sync::RwLockWriteGuard<'_, T> {
    *self.is_dirty.lock().unwrap() = true;
    self.data.write().unwrap()
  }
  pub fn read(&self) -> std::sync::RwLockReadGuard<'_, T> {
    self.data.read().unwrap()
  }
}
impl<T: BufferAttribute> UniformBufferTrait for UniformBuffer<T> {
  fn bind(&self, program: &RawShaderProgram) -> bool {
    {
      let mut is_dirty_lock = self.is_dirty.lock().unwrap();
      if *is_dirty_lock {
        self.raw_buffer.write_untyped(0, self.read().ub_data());
        *is_dirty_lock = false;
      }
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
