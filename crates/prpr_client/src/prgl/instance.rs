use super::*;

pub struct Instance {
  gl: Rc<WebGlContext>,
  max_width: i32,
  max_height: i32,
}

impl Instance {
  pub fn new(gl: gl) -> Self {
    // 一度生成したら固定
    let screen = html::screen();
    Self {
      gl: Rc::new(gl),
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
  // create gpu objects
  // pub fn new_shader(&self) {}
  // pub fn new_sampler(&self) {}
  // pub fn new_texture(&self) -> Texture {
  //   Texture {}
  // }
  // pub fn new_buffer(&self) -> Buffer {
  //   Buffer {}
  // }
  pub fn new_surface(&self) -> Texture {
    Texture {
      gl: Rc::clone(&self.gl),
    }
  }
  pub fn new_pipeline(&self) -> Pipeline {
    Pipeline::new(Rc::clone(&self.gl))
  }
  pub fn new_renderpass(&self) -> RenderPass {
    RenderPass::new(Rc::clone(&self.gl))
  }
}
