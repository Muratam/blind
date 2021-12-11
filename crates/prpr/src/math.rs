pub use glam::*;
use num_traits;

pub struct Rect<T: num_traits::Num> {
  pub x: T,
  pub y: T,
  pub width: T,
  pub height: T,
}
