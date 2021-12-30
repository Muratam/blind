pub use glam::*;
pub use num_traits;

#[derive(Clone)]
pub struct Rect<T: num_traits::Num + Copy> {
  pub x: T,
  pub y: T,
  pub width: T,
  pub height: T,
}
impl<T: num_traits::Num + Copy> Rect<T> {
  pub fn new(x: T, y: T, width: T, height: T) -> Self {
    Self {
      x,
      y,
      width,
      height,
    }
  }
}
impl<T: num_traits::NumCast + num_traits::Num + Copy> Rect<T> {
  pub fn aspect_ratio(&self) -> f32 {
    if let Some(width) = self.width.to_f32() {
      if let Some(height) = self.height.to_f32() {
        return width / height;
      }
    }
    return 1.0;
  }
}

#[derive(Clone, Copy)]
pub enum Color {
  Rgba(f32, f32, f32, f32),
  Hsla(f32, f32, f32, f32),
}
impl Color {
  pub fn rgba(rgba: Vec4) -> Self {
    Self::Rgba(rgba.x, rgba.y, rgba.z, rgba.w)
  }
  pub fn hsla(hsla: Vec4) -> Self {
    Self::Hsla(hsla.x, hsla.y, hsla.z, hsla.w)
  }
  pub fn to_css(&self) -> String {
    fn clamp255(x: f32) -> i32 {
      ((x * 255.0) as i32).clamp(0, 255)
    }
    match self {
      Self::Rgba(r, g, b, a) => {
        format!(
          "rgba({},{},{},{:.4})",
          clamp255(*r),
          clamp255(*g),
          clamp255(*b),
          a.clamp(0.0, 1.0)
        )
      }
      Self::Hsla(h, s, l, a) => {
        format!(
          "hsla({},{},{},{:.4})",
          clamp255(*h),
          clamp255(*s),
          clamp255(*l),
          a.clamp(0.0, 1.0)
        )
      }
    }
  }
}
