use super::*;

// 小さすぎると崩れるので、ある程度の大きさのheightを仮定している
pub const EXPECTED_BROWSER_HEIGHT: f32 = 1000.0;
pub fn convert_percent(x: f32) -> f32 {
  x * EXPECTED_BROWSER_HEIGHT * 0.01
}
pub fn convert_percent_str(x: f32) -> String {
  format!("{}px", convert_percent(x))
}

pub fn rgba_to_css_value(v: Vec4) -> String {
  fn clamp255(x: f32) -> i32 {
    ((x * 255.0) as i32).clamp(0, 255)
  }
  format!(
    "rgba({},{},{},{:.4})",
    clamp255(v.x),
    clamp255(v.y),
    clamp255(v.z),
    v.w.clamp(0.0, 1.0)
  )
}

pub enum Gradation {
  Linear(f32, Vec<Vec4>),            // degree, rgbas
  Radial(bool, f32, f32, Vec<Vec4>), // is_circle, x, y, rgbas
}
impl Gradation {
  pub fn to_css_value(&self) -> String {
    match self {
      Self::Linear(degree, rgbas) => {
        let mut result = format!("linear-gradient({}deg ", *degree as i32);
        for rgba in rgbas {
          result += &format!(", {}", rgba_to_css_value(*rgba));
        }
        result += ")";
        result
      }
      Self::Radial(is_circle, x, y, rgbas) => {
        let mut result = format!(
          "radial-gradient({} at {:.2}% {:.2}%, ",
          if *is_circle { "circle" } else { "ellipse" },
          *x * 100.0,
          *y * 100.0,
        );
        for rgba in rgbas {
          result += &format!(", {}", rgba_to_css_value(*rgba));
        }
        result += ")";
        result
      }
    }
  }
}

#[derive(Clone, Copy)]
pub struct FilterBlur(pub f32);
impl WhyTrait for FilterBlur {
  fn concat(&self, x: &Self) -> Self {
    Self(self.0 + x.0)
  }
}
impl FilterBlur {
  pub fn to_css_value(&self) -> String {
    format!("blur({})", convert_percent_str(self.0))
  }
}

#[derive(Clone, Copy)]
pub struct FilterDropShadow {
  pub x: f32,
  pub y: f32,
  pub r: f32,
  pub rgba: Vec4,
}
impl WhyTrait for FilterDropShadow {
  fn concat(&self, x: &Self) -> Self {
    Self {
      x: self.x + x.x,
      y: self.y + x.y,
      r: self.r + x.r,
      rgba: self.rgba * x.rgba,
    }
  }
}
impl FilterDropShadow {
  pub fn to_css_value(&self) -> String {
    format!(
      "drop-shadow({} {} {} {} ",
      convert_percent_str(self.x),
      convert_percent_str(self.y),
      convert_percent_str(self.r),
      rgba_to_css_value(self.rgba)
    )
  }
}

#[derive(Clone, Copy)]
pub struct FilterHueRotate(pub f32);
impl WhyTrait for FilterHueRotate {
  fn concat(&self, x: &Self) -> Self {
    Self(self.0 + x.0)
  }
}
impl FilterHueRotate {
  pub fn to_css_value(&self) -> String {
    format!("hue-rotate({:.4}deg)", self.0)
  }
}

#[derive(Clone, Copy)]
pub struct FilterBrightness(pub f32);
impl WhyTrait for FilterBrightness {
  fn concat(&self, x: &Self) -> Self {
    Self(self.0 * x.0)
  }
}
impl FilterBrightness {
  pub fn to_css_value(&self) -> String {
    format!("brightness({:.4})", self.0)
  }
}

#[derive(Clone, Copy)]
pub struct FilterContrast(pub f32);
impl WhyTrait for FilterContrast {
  fn concat(&self, x: &Self) -> Self {
    Self(self.0 * x.0)
  }
}
impl FilterContrast {
  pub fn to_css_value(&self) -> String {
    format!("contrast({:.4})", self.0)
  }
}

#[derive(Clone, Copy)]
pub struct FilterGrayscale(pub f32);
impl WhyTrait for FilterGrayscale {
  fn concat(&self, x: &Self) -> Self {
    Self(self.0 + x.0)
  }
}
impl FilterGrayscale {
  pub fn to_css_value(&self) -> String {
    format!("grayscale({:.4})", self.0)
  }
}

#[derive(Clone, Copy)]
pub struct FilterOpacity(pub f32);
impl WhyTrait for FilterOpacity {
  fn concat(&self, x: &Self) -> Self {
    Self(self.0 * x.0)
  }
}
impl FilterOpacity {
  pub fn to_css_value(&self) -> String {
    format!("opacity({:.4})", self.0)
  }
}

#[derive(Clone, Copy)]
pub enum BorderStyle {
  Solid,
  Double,
  Hidden,
  Dashed,
}
impl BorderStyle {
  pub fn to_css_value(&self) -> &'static str {
    match self {
      Self::Solid => "solid",
      Self::Double => "double",
      Self::Hidden => "hidden",
      Self::Dashed => "dashed",
    }
  }
}

#[derive(Clone, Copy)]
pub enum Align {
  Left,
  Right,
  Center,
}
impl Align {
  pub fn to_css_value(&self) -> &'static str {
    match self {
      Self::Left => "left",
      Self::Right => "right",
      Self::Center => "center",
    }
  }
}
#[derive(Clone, Copy)]
pub enum Cursor {
  Auto,
  Default,
  Pointer,
  Wait,
  Text,
  NotAllowed,
  Move,
  CrossHair,
  ColResize,
  RowResize,
}
impl Cursor {
  pub fn to_css_value(&self) -> &'static str {
    match self {
      Self::Auto => "auto",
      Self::Default => "default",
      Self::Pointer => "pointer",
      Self::Wait => "wait",
      Self::Text => "text",
      Self::NotAllowed => "not-allowed",
      Self::Move => "move",
      Self::CrossHair => "crosshair",
      Self::ColResize => "col-resize",
      Self::RowResize => "row-resize",
    }
  }
}

#[derive(Clone, Copy, PartialEq)]
pub enum TextDecorationStyle {
  Solid,
  Double,
  Dotted,
  Dashed,
  Wavy,
  None,
}
impl TextDecorationStyle {
  pub fn to_css_value(&self) -> &'static str {
    match self {
      Self::Solid => "solid",
      Self::Double => "double",
      Self::Dotted => "dotted",
      Self::Dashed => "dashed",
      Self::Wavy => "wavy",
      Self::None => "none",
    }
  }
}
