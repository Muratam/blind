use super::*;

pub struct TransformObject {
  pipeline: ArcOwner<Pipeline>,
  transform: Transform,
  scale_whys: Whys<TransformScaleData>,
  rotation_whys: Whys<TransformRotationData>,
  translate_whys: Whys<TransformTranslateData>,
}
impl TransformObject {
  pub fn new() -> Self {
    let mut pipeline = Pipeline::new();
    let transform = Transform::new();
    transform.bind_pipeline(&mut pipeline);
    Self {
      pipeline: ArcOwner::new(pipeline),
      transform,
      scale_whys: Whys::new(),
      rotation_whys: Whys::new(),
      translate_whys: Whys::new(),
    }
  }
  pub fn pipeline(&mut self) -> RwLockWriteGuard<'_, Pipeline> {
    self.pipeline.write()
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

impl RenderPassBindable for TransformObject {
  fn bind_renderpass(&self, renderpass: &mut RenderPass) {
    renderpass.add_pipeline(&self.pipeline);
  }
}
