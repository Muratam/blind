use super::*;
use crate::html;
pub struct Instance {
  gl: ArcGlContext,
  max_width: i32,
  max_height: i32,
}
impl Instance {
  pub fn new(gl: gl) -> Self {
    // 一度生成したら固定
    let screen = html::screen();
    Self {
      gl: Arc::new(gl),
      max_width: screen.width().unwrap(),
      max_height: screen.height().unwrap(),
    }
  }
  // 諸々更新が終わった後このテクスチャを利用する
  pub fn swap_surface(&self, surface: &Texture) {
    // WARN: surfaceテクスチャを使う
    let gl = &self.gl;
    gl.flush();
    // client_wait_sync ?
  }
  pub fn gl(&self) -> &ArcGlContext {
    &self.gl
  }
}
