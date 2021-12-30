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
pub fn from_hlsa(hlsa: Vec4) -> Vec4 {
  let h = (360.0 + (hlsa.x % 360.0)) % 360.0;
  let l = hlsa.y.clamp(0.0, 1.0);
  let l2 = if l > 0.5 { 1.0 - l } else { l };
  let s = hlsa.z.clamp(0.0, 1.0);
  let max = l + l2 * s;
  let min = l - l2 * s;
  let f = |x: f32| x / 60.0 * (max - min) + min;
  let a = hlsa.w;
  if h < 60.0 {
    Vec4::new(max, f(h), min, a)
  } else if h < 120.0 {
    Vec4::new(f(120.0 - h), max, min, a)
  } else if h < 180.0 {
    Vec4::new(min, max, f(h - 120.0), a)
  } else if h < 240.0 {
    Vec4::new(min, f(240.0 - h), max, a)
  } else if h < 300.0 {
    Vec4::new(f(h - 240.0), min, max, a)
  } else {
    Vec4::new(max, min, f(360.0 - h), a)
  }
}
