use super::*;
pub struct Mouse {}
impl Mouse {
  pub fn x() -> i32 {
    EventHolderImpl::read_global().mouse_x()
  }
  pub fn y() -> i32 {
    EventHolderImpl::read_global().mouse_y()
  }
  pub fn dx() -> i32 {
    EventHolderImpl::read_global().mouse_dx()
  }
  pub fn dy() -> i32 {
    EventHolderImpl::read_global().mouse_dy()
  }
  pub fn state(state: MouseState) -> bool {
    EventHolderImpl::read_global().mouse_state(state)
  }
  pub fn wheel_dx() -> i32 {
    EventHolderImpl::read_global().wheel_dx()
  }
  pub fn wheel_dy() -> i32 {
    EventHolderImpl::read_global().wheel_dy()
  }
}
