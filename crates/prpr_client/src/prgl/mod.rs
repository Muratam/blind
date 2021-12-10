// WebGlをラップしたもの
use crate::html;
use web_sys::WebGlRenderingContext as Gl;

pub struct Instance {
  gl: Gl,
  max_width: i32,
  max_height: i32,
}
// 実態として必要な情報を全て詰め込んだもの
struct RawTexture {}
struct RawBuffer {}
struct RawShader {}
struct RawSampler {}
struct RawPipeline {
  //// pipeline
// gl.enable(gl::DEPTH_TEST);
// gl.depth_func(gl::LEQUAL);
}
struct RawRenderPass {
  //// renderpass
// gl.viewport(0, 0, 10, 10);
// gl.clear_color(f, f, f, 0.2);
// gl.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
}

pub struct Texture {}
pub struct Buffer {}
pub struct RenderPass {}
pub struct Pipeline {}

impl Instance {
  pub fn new(gl: Gl) -> Self {
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
  pub fn new_shader(&self) {}
  pub fn new_sampler(&self) {}
  pub fn new_texture(&self) -> Texture {
    Texture {}
  }
  pub fn new_buffer(&self) -> Buffer {
    Buffer {}
  }
  pub fn new_pipeline(&self) -> Pipeline {
    Pipeline {}
  }
  pub fn new_renderpass(&self) -> RenderPass {
    RenderPass {}
  }
}
