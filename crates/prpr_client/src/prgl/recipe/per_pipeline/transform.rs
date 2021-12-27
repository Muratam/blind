use super::*;

crate::shader_attr! {
  struct TransformAttribute {
    model_mat: mat4
  }
}
pub struct Transform {
  ubo: Arc<UniformBuffer<TransformAttribute>>,
  pub scale: Vec3,
  pub rotation: Quat,
  pub translate: Vec3,
}
impl Transform {
  pub fn new(ctx: &ArcGlContext) -> Self {
    let ubo = Arc::new(UniformBuffer::new(
      ctx,
      TransformAttribute {
        model_mat: Mat4::IDENTITY,
      },
    ));
    Self {
      scale: Vec3::ONE,
      rotation: Quat::IDENTITY,
      translate: Vec3::ZERO,
      ubo,
    }
  }
  pub fn update(&mut self) {
    let model_mat =
      Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translate);
    let mut ubo = self.ubo.write_lock();
    ubo.model_mat = model_mat;
  }
}
impl PipelineBindable for Transform {
  fn bind(&self, pipeline: &mut Pipeline) {
    pipeline.add_uniform_buffer(&self.ubo);
  }
}
