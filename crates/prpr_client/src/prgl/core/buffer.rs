use super::*;
// xN のGpuバッファを内部に持つ？
// -> デフォルトは1で後からResizeを設定、くらいの流れで
// -> 差はここで吸収

pub struct IndexBuffer {
  raw_buffer: RawBuffer,
  data: Vec<IndexBufferType>,
}
impl IndexBuffer {
  pub fn new(data: Vec<IndexBufferType>) -> Self {
    Self {
      raw_buffer: RawBuffer::new(data.as_slice(), BufferUsage::Index),
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
  pub fn new(data: Vec<T>) -> Self {
    let template = if data.len() > 0 {
      data[0].vs_in_template()
    } else {
      Default::default()
    };
    Self {
      raw_buffer: RawBuffer::new(data.as_slice(), BufferUsage::Vertex),
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

pub trait UniformBufferTrait {
  fn bind(&self, cmd: &mut Command);
}
pub struct UniformBuffer<T: BufferAttribute> {
  raw_buffer: RawBuffer,
  data: T,
  name: &'static str,
  is_dirty: Mutex<bool>,
}
impl<T: BufferAttribute> UniformBuffer<T> {
  pub fn new(data: T) -> Self {
    Self {
      name: data.name(),
      raw_buffer: RawBuffer::new_untyped(data.ub_data(), BufferUsage::Uniform),
      is_dirty: Mutex::new(false),
      data,
    }
  }
}

impl<T: BufferAttribute> std::ops::Deref for UniformBuffer<T> {
  type Target = T;
  fn deref(&self) -> &T {
    &self.data
  }
}
impl<T: BufferAttribute> std::ops::DerefMut for UniformBuffer<T> {
  fn deref_mut(&mut self) -> &mut T {
    *self.is_dirty.lock().unwrap() = true;
    &mut self.data
  }
}

impl<T: BufferAttribute> UniformBufferTrait for UniformBuffer<T> {
  fn bind(&self, cmd: &mut Command) {
    {
      let mut is_dirty_lock = self.is_dirty.lock().unwrap();
      if !*is_dirty_lock {
        self.raw_buffer.write_untyped(0, self.data.ub_data());
        *is_dirty_lock = false;
      }
    }
    if let Some(shader) = cmd.current_shader() {
      if let Some(index) = shader.uniform_block_index(self.name) {
        cmd.set_ubo(&self.raw_buffer, index);
      }
    }
  }
}
impl<T: BufferAttribute> UniformBufferTrait for Reader<UniformBuffer<T>> {
  fn bind(&self, cmd: &mut Command) {
    self.read().bind(cmd);
  }
}

pub trait RefInto<T> {
  fn ref_into(&self) -> T;
}
pub struct IntoUniformBuffer<T: BufferAttribute, I: RefInto<T>> {
  raw_buffer: RawBuffer,
  name: &'static str,
  phantom_data: std::marker::PhantomData<T>,
  into: I,
  is_dirty: Mutex<bool>,
}
impl<T: BufferAttribute, I: RefInto<T>> IntoUniformBuffer<T, I> {
  pub fn new(into: I) -> Self {
    let data = (&into).ref_into();
    Self {
      name: data.name(),
      raw_buffer: RawBuffer::new_untyped(data.ub_data(), BufferUsage::Uniform),
      is_dirty: Mutex::new(true),
      phantom_data: std::marker::PhantomData,
      into: into,
    }
  }
}
impl<T: BufferAttribute, I: RefInto<T>> std::ops::Deref for IntoUniformBuffer<T, I> {
  type Target = I;
  fn deref(&self) -> &I {
    &self.into
  }
}
impl<T: BufferAttribute, I: RefInto<T>> std::ops::DerefMut for IntoUniformBuffer<T, I> {
  fn deref_mut(&mut self) -> &mut I {
    *self.is_dirty.lock().unwrap() = true;
    &mut self.into
  }
}

impl<T: BufferAttribute, I: RefInto<T>> UniformBufferTrait for IntoUniformBuffer<T, I> {
  fn bind(&self, cmd: &mut Command) {
    {
      let mut is_dirty_lock = self.is_dirty.lock().unwrap();
      if *is_dirty_lock {
        let data: T = self.into.ref_into();
        self.raw_buffer.write_untyped(0, data.ub_data());
        *is_dirty_lock = false;
      }
    }
    if let Some(shader) = cmd.current_shader() {
      if let Some(index) = shader.uniform_block_index(self.name) {
        cmd.set_ubo(&self.raw_buffer, index);
      }
    }
  }
}
impl<T: BufferAttribute, I: RefInto<T>> UniformBufferTrait for Reader<IntoUniformBuffer<T, I>> {
  fn bind(&self, cmd: &mut Command) {
    self.read().bind(cmd);
  }
}
