use super::*;
pub use event::MouseState;
pub struct Mouse {}
impl Mouse {
  pub fn state(state: MouseState) -> bool {
    EventHolderImpl::read_global().mouse_state(state)
  }
  // 中央が原点で左上が正
  pub fn x() -> i32 {
    let width = WholeScreen::width();
    EventHolderImpl::read_global().mouse_x() - width / 2
  }
  pub fn y() -> i32 {
    let height = WholeScreen::height();
    height / 2 - EventHolderImpl::read_global().mouse_y()
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
    let height = WholeScreen::height();
    Self::x() as f32 / height as f32
  }
  pub fn ny() -> f32 {
    let height = WholeScreen::height();
    Self::y() as f32 / height as f32
  }
  pub fn dnx() -> f32 {
    let height = WholeScreen::height();
    Self::dx() as f32 / height as f32
  }
  pub fn dny() -> f32 {
    let height = WholeScreen::height();
    Self::dy() as f32 / height as f32
  }
  pub fn wheel_dnx() -> f32 {
    let height = WholeScreen::height();
    Self::wheel_dx() as f32 / height as f32
  }
  pub fn wheel_dny() -> f32 {
    let height = WholeScreen::height();
    Self::wheel_dy() as f32 / height as f32
  }
}
