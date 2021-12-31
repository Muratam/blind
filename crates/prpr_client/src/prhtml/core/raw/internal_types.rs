use super::*;

#[derive(Clone, Copy)]
pub struct HtmlTransform {
  pub scale: f32,
  pub rotate_deg: f32,
  pub translate: Vec2,
}
impl WhyTrait for HtmlTransform {
  fn concat(&self, x: &Self) -> Self {
    Self {
      scale: self.scale * x.scale,
      rotate_deg: self.rotate_deg + x.rotate_deg,
      translate: self.translate + x.translate,
    }
  }
}
impl Default for HtmlTransform {
  fn default() -> Self {
    Self {
      scale: 1.0,
      rotate_deg: 0.0,
      translate: Vec2::ZERO,
    }
  }
}

impl HtmlTransform {
  pub fn to_css_value(&self) -> String {
    format!(
      "translate({},{}) scale({}) rotate({}deg)",
      convert_percent_str(self.translate.x),
      convert_percent_str(self.translate.y),
      self.scale,
      self.rotate_deg
    )
  }
}
