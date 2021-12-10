// WebGlをラップしたもの
mod raw_type;
use crate::html;
use raw_type::*;
pub struct Instance {
  gl: Gl,
  max_width: i32,
  max_height: i32,
}
pub struct Texture {}
pub struct Buffer {}
pub struct Pipeline {}
impl Pipeline {
  pub fn draw(&self) {}
}
pub struct RenderPass {}
impl RenderPass {
  pub fn bind(&self) {}
}

impl Instance {
  pub fn new(gl: Gl) -> Self {
    // 一度生成したら固定
    let screen = html::screen();
    Self {
      gl,
      max_width: screen.width().unwrap(),
      max_height: screen.height().unwrap(),
    }
  }
  // 諸々更新が終わった後このテクスチャを利用する
  pub fn update(&self, surface: &Texture) {}
  // create gpu objects
  // pub fn new_shader(&self) {}
  // pub fn new_sampler(&self) {}
  // pub fn new_texture(&self) -> Texture {
  //   Texture {}
  // }
  // pub fn new_buffer(&self) -> Buffer {
  //   Buffer {}
  // }
  pub fn new_sandbox_surface(&self) -> Texture {
    Texture {}
  }
  pub fn new_sandbox_pipeline(&self) -> Pipeline {
    Pipeline {}
  }
  pub fn new_sandbox_renderpass(&self) -> RenderPass {
    RenderPass {}
  }
}
