use super::*;

crate::shader_attr! {
  struct TransformAttribute {
    model_mat: mat4
  }
}
#[derive(Clone)]
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

// WHY
#[derive(Clone)]
pub struct TransformScaleData(pub Vec3);
#[derive(Clone)]
pub struct TransformRotationData(pub Quat);
#[derive(Clone)]
pub struct TransformTranslateData(pub Vec3);
impl WhyTrait for TransformScaleData {
  fn concat(&self, x: &Self) -> Self {
    Self(self.0 * x.0)
  }
}
impl WhyTrait for TransformRotationData {
  fn concat(&self, x: &Self) -> Self {
    Self(self.0 * x.0)
  }
}
impl WhyTrait for TransformTranslateData {
  fn concat(&self, x: &Self) -> Self {
    Self(self.0 + x.0)
  }
}
impl Default for TransformScaleData {
  fn default() -> Self {
    Self(Vec3::ONE)
  }
}
impl Default for TransformRotationData {
  fn default() -> Self {
    Self(Quat::IDENTITY)
  }
}
impl Default for TransformTranslateData {
  fn default() -> Self {
    Self(Vec3::ZERO)
  }
}
