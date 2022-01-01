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
pub struct TransformWhy {
  transform: Transform,
  scale_whys: Whys<TransformScaleData>,
  rotation_whys: Whys<TransformRotationData>,
  translate_whys: Whys<TransformTranslateData>,
}
impl TransformWhy {
  pub fn new() -> Self {
    let transform = Transform::new();
    Self {
      transform,
      scale_whys: Whys::new(),
      rotation_whys: Whys::new(),
      translate_whys: Whys::new(),
    }
  }
  pub fn scale(&self) -> Vec3 {
    self.scale_whys.calc_or_default().0
  }
  pub fn rotation(&self) -> Quat {
    self.rotation_whys.calc_or_default().0
  }
  pub fn translate(&self) -> Vec3 {
    self.translate_whys.calc_or_default().0
  }
  pub fn ask_scale(&self, why: Why) -> Vec3 {
    self.scale_whys.get(why).unwrap_or_default().0
  }
  pub fn ask_rotation(&self, why: Why) -> Quat {
    self.rotation_whys.get(why).unwrap_or_default().0
  }
  pub fn ask_translate(&self, why: Why) -> Vec3 {
    self.translate_whys.get(why).unwrap_or_default().0
  }

  pub fn set_scale(&mut self, scale: Vec3, why: Why) {
    self.scale_whys.set(Some(TransformScaleData(scale)), why);
    self.transform.write().scale = self.scale_whys.calc_or_default().0;
  }
  pub fn set_rotation(&mut self, rotation: Quat, why: Why) {
    self
      .rotation_whys
      .set(Some(TransformRotationData(rotation)), why);
    self.transform.write().rotation = self.rotation_whys.calc_or_default().0;
  }
  pub fn set_translate(&mut self, translate: Vec3, why: Why) {
    self
      .translate_whys
      .set(Some(TransformTranslateData(translate)), why);
    self.transform.write().translate = self.translate_whys.calc_or_default().0;
  }
}
impl PipelineBindable for TransformWhy {
  fn bind_pipeline(&self, pipeline: &mut Pipeline) {
    pipeline.add(&self.transform);
  }
}
impl Default for TransformWhy {
  fn default() -> Self {
    Self::new()
  }
}
