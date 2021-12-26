use super::*;

pub struct TransformObject {
  pub pipeline: Pipeline, // TODO: not pub
  transform: Transform,
}
impl TransformObject {
  pub fn new(ctx: &ArcGlContext) -> Self {
    let mut pipeline = Pipeline::new(ctx);
    let transform = Transform::new(ctx);
    transform.bind(&mut pipeline);
    Self {
      pipeline,
      transform,
    }
  }
  pub fn add(&mut self, bindable: &dyn PipelineBindable) {
    bindable.bind(&mut self.pipeline);
  }
}
