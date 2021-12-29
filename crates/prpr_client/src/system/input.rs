use super::*;
pub use event::MouseState;
pub struct Mouse {}
impl Mouse {
  pub fn state(state: MouseState) -> bool {
    EventHolderImpl::read_global().mouse_state(state)
  }
  // CanvasのFrameBufferと合わせて左下が原点
  pub fn x() -> i32 {
    EventHolderImpl::read_global().mouse_x()
  }
  pub fn y() -> i32 {
    let height = prgl::Instance::viewport().height;
    height - EventHolderImpl::read_global().mouse_y()
  }
  pub fn dx() -> i32 {
    EventHolderImpl::read_global().mouse_dx()
  }
  pub fn dy() -> i32 {
    -EventHolderImpl::read_global().mouse_dy()
  }
  pub fn wheel_dx() -> i32 {
    EventHolderImpl::read_global().wheel_dx()
  }
  pub fn wheel_dy() -> i32 {
    -EventHolderImpl::read_global().wheel_dy()
  }
  // normalized by viewport_y
  pub fn nx() -> f32 {
    let height = prgl::Instance::viewport().height;
    Self::x() as f32 / height as f32
  }
  pub fn ny() -> f32 {
    let height = prgl::Instance::viewport().height;
    Self::y() as f32 / height as f32
  }
  pub fn dnx() -> f32 {
    let height = prgl::Instance::viewport().height;
    Self::dx() as f32 / height as f32
  }
  pub fn dny() -> f32 {
    let height = prgl::Instance::viewport().height;
    Self::dy() as f32 / height as f32
  }
  pub fn wheel_dnx() -> f32 {
    let height = prgl::Instance::viewport().height;
    Self::wheel_dx() as f32 / height as f32
  }
  pub fn wheel_dny() -> f32 {
    let height = prgl::Instance::viewport().height;
    Self::wheel_dy() as f32 / height as f32
  }
}
