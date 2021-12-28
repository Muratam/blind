use super::*;

pub struct TransformObject {
  pub pipeline: Pipeline,
  pub transform: Transform,
}
impl TransformObject {
  pub fn new(ctx: &ArcGlContext) -> Self {
    let mut pipeline = Pipeline::new(ctx);
    let transform = Transform::new(ctx);
    transform.bind_pipeline(&mut pipeline);
    Self {
      pipeline,
      transform,
    }
  }
}
