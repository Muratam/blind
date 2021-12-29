use super::*;

pub struct TransformObject {
  pub pipeline: Owner<Pipeline>,
  pub transform: Transform,
}
impl TransformObject {
  pub fn new() -> Self {
    let mut pipeline = Pipeline::new();
    let transform = Transform::new();
    transform.bind_pipeline(&mut pipeline);
    Self {
      pipeline: Owner::new(pipeline),
      transform,
    }
  }
}
