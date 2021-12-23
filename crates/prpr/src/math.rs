pub use glam::*;
pub use num_traits;

#[derive(Clone)]
pub struct Rect<T: num_traits::Num> {
  pub x: T,
  pub y: T,
  pub width: T,
  pub height: T,
}

impl<T: num_traits::Num> Rect<T> {
  pub fn new(x: T, y: T, width: T, height: T) -> Self {
    Self {
      x,
      y,
      width,
      height,
    }
  }
}
