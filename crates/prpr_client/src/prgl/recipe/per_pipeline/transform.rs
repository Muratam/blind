use super::*;

crate::shader_attr! {
  struct TransformAttribute {
    model_mat: mat4
  }
}
pub struct TransformData {
  pub scale: Vec3,
  pub rotation: Quat,
  pub translate: Vec3,
}
impl Default for TransformData {
  fn default() -> Self {
    Self {
      scale: Vec3::ONE,
      rotation: Quat::IDENTITY,
      translate: Vec3::ZERO,
    }
  }
}
impl RefInto<TransformAttribute> for TransformData {
  fn ref_into(&self) -> TransformAttribute {
    TransformAttribute {
      model_mat: Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translate),
    }
  }
}

pub type Transform = IntoUniformBufferTemplate<TransformAttribute, TransformData>;
