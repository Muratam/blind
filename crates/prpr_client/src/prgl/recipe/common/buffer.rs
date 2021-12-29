use super::*;

pub struct UniformBufferTemplate<T: BufferAttribute + Default> {
  data: Primary<UniformBuffer<T>>,
}
impl<T: BufferAttribute + Default> UniformBufferTemplate<T> {
  pub fn new() -> Self {
    Self {
      data: Primary::new(UniformBuffer::new(Default::default())),
    }
  }
  pub fn write(&mut self) -> RwLockWriteGuard<'_, UniformBuffer<T>> {
    self.data.write()
  }
  pub fn read(&self) -> RwLockReadGuard<'_, UniformBuffer<T>> {
    self.data.read()
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
  data: Primary<IntoUniformBuffer<T, I>>,
}
impl<T: BufferAttribute, I: RefInto<T> + Default> IntoUniformBufferTemplate<T, I> {
  pub fn new() -> Self {
    Self {
      data: Primary::new(IntoUniformBuffer::new(Default::default())),
    }
  }
  pub fn write(&mut self) -> RwLockWriteGuard<'_, IntoUniformBuffer<T, I>> {
    self.data.write()
  }
  pub fn read(&self) -> RwLockReadGuard<'_, IntoUniformBuffer<T, I>> {
    self.data.read()
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
