use super::*;

#[derive(Clone, Copy)]
pub struct HtmlTransform {
  pub scale: f32,
  pub rotate_degree: f32,
  pub offset: Vec2,
}
impl WhyTrait for HtmlTransform {
  fn concat(&self, x: Self) -> Self {
    Self {
      scale: self.scale * x.scale,
      rotate_degree: self.rotate_degree + x.rotate_degree,
      offset: self.offset + x.offset,
    }
  }
}
