use num_traits;

pub struct Vec4 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32,
}
pub struct Rect<T: num_traits::Num> {
  pub x: T,
  pub y: T,
  pub width: T,
  pub height: T,
}
