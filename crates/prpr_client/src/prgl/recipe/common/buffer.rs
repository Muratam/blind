use super::*;

pub struct UniformBufferTemplate<T: BufferAttribute + Default> {
  data: SOwner<UniformBuffer<T>>,
}
impl<T: BufferAttribute + Default> UniformBufferTemplate<T> {
  pub fn new() -> Self {
    Self {
      data: SOwner::new(UniformBuffer::new(Default::default())),
    }
  }
  pub fn write(&mut self) -> SDerefMutable<'_, UniformBuffer<T>> {
    self.data.write()
  }
  pub fn read(&self) -> SDerefable<'_, UniformBuffer<T>> {
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
  data: SOwner<IntoUniformBuffer<T, I>>,
}
impl<T: BufferAttribute, I: RefInto<T> + Default> IntoUniformBufferTemplate<T, I> {
  pub fn new() -> Self {
    Self {
      data: SOwner::new(IntoUniformBuffer::new(Default::default())),
    }
  }
  pub fn write(&mut self) -> SDerefMutable<'_, IntoUniformBuffer<T, I>> {
    self.data.write()
  }
  pub fn read(&self) -> SDerefable<'_, IntoUniformBuffer<T, I>> {
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
