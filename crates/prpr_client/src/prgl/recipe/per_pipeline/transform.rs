use super::*;

crate::shader_attr! {
  struct TransformAttribute {
    model_mat: mat4
  }
}
pub struct Transform {
  pub ubo: Arc<UniformBuffer<TransformAttribute>>,
}
impl Transform {
  pub fn new(ctx: &ArcGlContext) -> Self {
    let ubo = Arc::new(UniformBuffer::new(
      ctx,
      TransformAttribute {
        model_mat: Mat4::IDENTITY,
      },
    ));
    Self { ubo }
  }
}
