use super::*;
// xN のGpuバッファを内部に持つ？
// -> デフォルトは1で後からResizeを設定、くらいの流れで
// -> 差はここで吸収

pub struct IndexBuffer {
  raw_buffer: RawBuffer,
  data: Vec<IndexBufferType>,
}
impl IndexBuffer {
  pub fn new(ctx: &ArcGlContext, data: Vec<IndexBufferType>) -> Self {
    Self {
      raw_buffer: RawBuffer::new(ctx, data.as_slice(), BufferUsage::Index),
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
  pub fn new(ctx: &ArcGlContext, data: Vec<T>) -> Self {
    let template = if data.len() > 0 {
      data[0].vs_in_template()
    } else {
      Default::default()
    };
    Self {
      raw_buffer: RawBuffer::new(ctx, data.as_slice(), BufferUsage::Vertex),
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
  // returns successed
  fn bind(&self, shader: &Shader);
}
pub struct UniformBuffer<T: BufferAttribute> {
  ctx: ArcGlContext,
  raw_buffer: RawBuffer,
  data: RwLock<T>,
  name: &'static str,
  is_dirty: Mutex<bool>,
}
impl<T: BufferAttribute> UniformBuffer<T> {
  pub fn new(ctx: &ArcGlContext, data: T) -> Self {
    Self {
      ctx: ctx.clone(),
      name: data.name(),
      raw_buffer: RawBuffer::new_untyped(ctx, data.ub_data(), BufferUsage::Uniform),
      is_dirty: Mutex::new(false),
      data: RwLock::new(data),
    }
  }
  pub fn write_lock(&self) -> std::sync::RwLockWriteGuard<'_, T> {
    *self.is_dirty.lock().unwrap() = true;
    self.data.write().unwrap()
  }
  pub fn read_lock(&self) -> std::sync::RwLockReadGuard<'_, T> {
    self.data.read().unwrap()
  }
}
impl<T: BufferAttribute> UniformBufferTrait for UniformBuffer<T> {
  fn bind(&self, shader: &Shader) {
    {
      let mut is_dirty_lock = self.is_dirty.lock().unwrap();
      if *is_dirty_lock {
        self.raw_buffer.write_untyped(0, self.read_lock().ub_data());
        *is_dirty_lock = false;
      }
    }
    if let Some(index) = shader.uniform_block_index(self.name) {
      self.ctx.bind_buffer_base(
        gl::UNIFORM_BUFFER,
        index,
        Some(self.raw_buffer.raw_buffer()),
      );
    }
  }
}
pub trait RefInto<T> {
  fn ref_into(&self) -> T;
}
pub struct IntoUniformBuffer<T: BufferAttribute, I: RefInto<T>> {
  ctx: ArcGlContext,
  raw_buffer: RawBuffer,
  name: &'static str,
  phantom_data: std::marker::PhantomData<T>,
  into: RwLock<I>,
  is_dirty: Mutex<bool>,
}
impl<T: BufferAttribute, I: RefInto<T>> IntoUniformBuffer<T, I> {
  pub fn new(ctx: &ArcGlContext, into: I) -> Self {
    let data = (&into).ref_into();
    Self {
      ctx: ctx.clone(),
      name: data.name(),
      raw_buffer: RawBuffer::new_untyped(ctx, data.ub_data(), BufferUsage::Uniform),
      is_dirty: Mutex::new(true),
      phantom_data: std::marker::PhantomData,
      into: RwLock::new(into),
    }
  }
  pub fn write_lock(&self) -> std::sync::RwLockWriteGuard<'_, I> {
    *self.is_dirty.lock().unwrap() = true;
    self.into.write().unwrap()
  }
  pub fn read_lock(&self) -> std::sync::RwLockReadGuard<'_, I> {
    self.into.read().unwrap()
  }
}

impl<T: BufferAttribute, I: RefInto<T>> UniformBufferTrait for IntoUniformBuffer<T, I> {
  fn bind(&self, shader: &Shader) {
    {
      let mut is_dirty_lock = self.is_dirty.lock().unwrap();
      if *is_dirty_lock {
        let data: T = self.read_lock().ref_into();
        self.raw_buffer.write_untyped(0, data.ub_data());
        *is_dirty_lock = false;
      }
    }
    if let Some(index) = shader.uniform_block_index(self.name) {
      self.ctx.bind_buffer_base(
        gl::UNIFORM_BUFFER,
        index,
        Some(self.raw_buffer.raw_buffer()),
      );
    }
  }
}
