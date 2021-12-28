use super::*;

pub struct TransformObject {
  pub pipeline: Arc<RwLock<Pipeline>>,
  pub transform: Transform,
}
impl TransformObject {
  pub fn new() -> Self {
    let mut pipeline = Pipeline::new();
    let transform = Transform::new();
    transform.bind_pipeline(&mut pipeline);
    Self {
      pipeline: Arc::new(RwLock::new(pipeline)),
      transform,
    }
  }
  pub fn add(&self, bindable: &dyn PipelineBindable) {
    self.pipeline.write().unwrap().add(bindable);
  }
}
