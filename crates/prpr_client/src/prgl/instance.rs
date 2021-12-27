use super::*;
use crate::html;

pub struct Instance {
  ctx: ArcGlContext,
  max_width: i32,
  max_height: i32,
  width: i32,
  height: i32,
}
impl Instance {
  pub fn new(ctx: web_sys::WebGl2RenderingContext) -> Self {
    // 一度生成したら固定
    let screen = html::screen();
    Self {
      ctx: Arc::new(ctx),
      max_width: screen.width().unwrap(),
      max_height: screen.height().unwrap(),
      width: 1,
      height: 1,
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
    self.width
  }
  pub fn height(&self) -> i32 {
    self.height
  }
  pub fn full_viewport(&self) -> Rect<i32> {
    Rect::new(0, 0, self.width, self.height)
  }
  pub fn aspect_ratio(&self) -> f32 {
    self.width as f32 / self.height as f32
  }
  pub fn update_size(&mut self, width: i32, height: i32) {
    self.width = width;
    self.height = height;
  }
}
