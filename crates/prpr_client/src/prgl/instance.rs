use super::*;

// WARN: 多分別スレッドから実行できない
use once_cell::sync::OnceCell;
static INSTANCE: OnceCell<Instance> = OnceCell::new();
unsafe impl Send for Instance {}
unsafe impl Sync for Instance {}

pub struct Instance {
  ctx: web_sys::WebGl2RenderingContext,
  max_width: i32,
  max_height: i32,
  width: RwLock<i32>,
  height: RwLock<i32>,
}
impl Instance {
  pub fn get() -> &'static Self {
    INSTANCE.get().expect("prgl::Instance is not initialized")
  }
  pub fn ctx() -> &'static web_sys::WebGl2RenderingContext {
    &Self::get().ctx
  }
  pub fn set(ctx: web_sys::WebGl2RenderingContext) {
    // 一度生成したら固定
    let screen = js::html::screen();
    let instance = Self {
      ctx,
      max_width: screen.width().unwrap(),
      max_height: screen.height().unwrap(),
      width: RwLock::new(1),
      height: RwLock::new(1),
    };
    INSTANCE.set(instance).ok();
  }
  pub fn flush() {
    Self::ctx().flush();
  }
  pub fn max_width() -> i32 {
    Self::get().max_width
  }
  pub fn max_height() -> i32 {
    Self::get().max_height
  }
  pub fn width() -> i32 {
    *Self::get().width.read().unwrap()
  }
  pub fn height() -> i32 {
    *Self::get().height.read().unwrap()
  }
  pub fn viewport() -> Rect<i32> {
    let width = Self::width();
    let height = Self::height();
    Rect::new(
      (Self::max_width() - width) / 2,
      (Self::max_height() - height) / 2,
      width,
      height,
    )
  }
  pub fn max_viewport() -> Rect<i32> {
    Rect::new(0, 0, Self::max_width(), Self::max_height())
  }
  pub fn update_size(width: i32, height: i32) {
    *Self::get().width.write().unwrap() = width;
    *Self::get().height.write().unwrap() = height;
  }
}
