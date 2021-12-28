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
