use super::*;
use crate::html;

pub struct Instance {
  ctx: ArcGlContext,
  max_width: i32,
  max_height: i32,
  width: RwLock<i32>,
  height: RwLock<i32>,
}
impl Instance {
  pub fn new(ctx: web_sys::WebGl2RenderingContext) -> Self {
    // 一度生成したら固定
    let screen = html::screen();
    Self {
      ctx: Arc::new(ctx),
      max_width: screen.width().unwrap(),
      max_height: screen.height().unwrap(),
      width: RwLock::new(1),
      height: RwLock::new(1),
    }
  }
  pub fn flush(&self) {
    let ctx = &self.ctx;
    ctx.flush();
  }
  pub fn ctx(&self) -> &ArcGlContext {
    &self.ctx
  }
  pub fn max_width(&self) -> i32 {
    self.max_width
  }
  pub fn max_height(&self) -> i32 {
    self.max_height
  }
  pub fn width(&self) -> i32 {
    *self.width.read().unwrap()
  }
  pub fn height(&self) -> i32 {
    *self.height.read().unwrap()
  }
  pub fn full_viewport(&self) -> Rect<i32> {
    Rect::new(0, 0, self.width(), self.height())
  }
  pub fn full_max_viewport(&self) -> Rect<i32> {
    Rect::new(0, 0, self.max_width(), self.max_height())
  }
  pub fn update_size(&self, width: i32, height: i32) {
    *self.width.write().unwrap() = width;
    *self.height.write().unwrap() = height;
  }
}
