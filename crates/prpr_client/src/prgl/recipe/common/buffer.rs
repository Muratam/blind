use super::*;

pub struct UniformBufferTemplate<T: BufferAttribute + Default> {
  data: Arc<UniformBuffer<T>>,
}
impl<T: BufferAttribute + Default> UniformBufferTemplate<T> {
  pub fn new() -> Self {
    Self {
      data: Arc::new(UniformBuffer::new(Default::default())),
    }
  }
  pub fn write_lock(&self) -> RwLockWriteGuard<'_, T> {
    self.data.write_lock()
  }
  pub fn read_lock(&self) -> RwLockReadGuard<'_, T> {
    self.data.read_lock()
  }
}
impl<T: BufferAttribute + Default + 'static> PipelineBindable for UniformBufferTemplate<T> {
  fn bind_pipeline(&self, pipeline: &mut Pipeline) {
    pipeline.add_uniform_buffer(&self.data);
  }
}
impl<T: BufferAttribute + Default + 'static> RenderPassBindable for UniformBufferTemplate<T> {
  fn bind_renderpass(&self, renderpass: &mut RenderPass) {
    renderpass.add_uniform_buffer(&self.data);
  }
}

pub struct IntoUniformBufferTemplate<T: BufferAttribute, I: RefInto<T> + Default> {
  data: Arc<IntoUniformBuffer<T, I>>,
}
impl<T: BufferAttribute, I: RefInto<T> + Default> IntoUniformBufferTemplate<T, I> {
  pub fn new() -> Self {
    Self {
      data: Arc::new(IntoUniformBuffer::new(Default::default())),
    }
  }
  pub fn write_lock(&self) -> RwLockWriteGuard<'_, I> {
    self.data.write_lock()
  }
  pub fn read_lock(&self) -> RwLockReadGuard<'_, I> {
    self.data.read_lock()
  }
}
impl<T: BufferAttribute + 'static, I: RefInto<T> + 'static + Default> PipelineBindable
  for IntoUniformBufferTemplate<T, I>
{
  fn bind_pipeline(&self, pipeline: &mut Pipeline) {
    pipeline.add_into_uniform_buffer(&self.data);
  }
}
impl<T: BufferAttribute + 'static, I: RefInto<T> + 'static + Default> RenderPassBindable
  for IntoUniformBufferTemplate<T, I>
{
  fn bind_renderpass(&self, renderpass: &mut RenderPass) {
    renderpass.add_into_uniform_buffer(&self.data);
  }
}
