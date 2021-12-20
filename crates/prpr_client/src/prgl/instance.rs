use super::*;
use crate::html;
pub struct Instance {
  ctx: ArcGlContext,
  max_width: i32,
  max_height: i32,
}
impl Instance {
  pub fn new(ctx: web_sys::WebGl2RenderingContext) -> Self {
    // 一度生成したら固定
    let screen = html::screen();
    Self {
      ctx: Arc::new(ctx),
      max_width: screen.width().unwrap(),
      max_height: screen.height().unwrap(),
    }
  }
  // 諸々更新が終わった後このテクスチャを利用する
  pub fn swap_surface(&self, surface: &Texture) {
    // WARN: surfaceテクスチャを使う
    let ctx = &self.ctx;
    ctx.flush();
    // client_wait_sync ?
  }
  pub fn ctx(&self) -> &ArcGlContext {
    &self.ctx
  }
}
