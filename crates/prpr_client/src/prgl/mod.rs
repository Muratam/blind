// WebGlをラップしたもの
mod raw_type;
use crate::html;
use raw_type::*;
use std::rc::Rc;
pub struct Instance {
  gl: Rc<WebGlContext>,
  max_width: i32,
  max_height: i32,
}
pub struct Texture {
  gl: Rc<WebGlContext>,
}
pub struct Buffer {
  gl: Rc<WebGlContext>,
}
pub struct Pipeline {
  gl: Rc<WebGlContext>,
}
impl Pipeline {
  pub fn draw(&self) {}
}
pub struct RenderPass {
  gl: Rc<gl>,
}
impl RenderPass {
  pub fn bind(&self) {
    let gl = &self.gl;
    gl.clear_color(0.5, 0.5, 0.5, 1.0);
    gl.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
  }
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
  pub fn update(&self, surface: &Texture) {
    let gl = &self.gl;
    // gl.flush();
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
  pub fn new_sandbox_surface(&self) -> Texture {
    Texture {
      gl: Rc::clone(&self.gl),
    }
  }
  pub fn new_sandbox_pipeline(&self) -> Pipeline {
    Pipeline {
      gl: Rc::clone(&self.gl),
    }
  }
  pub fn new_sandbox_renderpass(&self) -> RenderPass {
    RenderPass {
      gl: Rc::clone(&self.gl),
    }
  }
}
