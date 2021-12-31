use super::*;

pub struct TransformObject {
  pipeline: ArcOwner<Pipeline>,
  pub transform: Transform,
}
impl TransformObject {
  pub fn new() -> Self {
    let mut pipeline = Pipeline::new();
    let transform = Transform::new();
    transform.bind_pipeline(&mut pipeline);
    Self {
      pipeline: ArcOwner::new(pipeline),
      transform,
    }
  }
  pub fn pipeline(&mut self) -> RwLockWriteGuard<'_, Pipeline> {
    self.pipeline.write()
  }
}

impl RenderPassBindable for TransformObject {
  fn bind_renderpass(&self, renderpass: &mut RenderPass) {
    renderpass.add_pipeline(&self.pipeline);
  }
}
