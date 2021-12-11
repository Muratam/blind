// WebGlをラップしたもの
mod raw_type;
use crate::html;
use prpr::math::*;
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
pub const MAX_OUTPUT_SLOT: usize = 8;
pub struct Pipeline {
  gl: Rc<WebGlContext>,
}
impl Pipeline {
  pub fn draw(&self) {}
}
pub struct RenderPass {
  gl: Rc<WebGlContext>,
  clear_colors: [Vec4; MAX_OUTPUT_SLOT],
}
impl RenderPass {
  pub fn new(gl: Rc<WebGlContext>) -> RenderPass {
    RenderPass {
      gl: Rc::clone(&gl),
      clear_colors: [Vec4::ZERO; MAX_OUTPUT_SLOT],
    }
  }
  pub fn bind(&self) {
    let gl = &self.gl;
    // TODO: 今はゼロスロット目のみ
    let color = self.clear_colors[0];
    gl.clear_color(color.x, color.y, color.z, color.w);
    gl.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
  }
  pub fn set_clear_color(&mut self, clear_color: Vec4) {
    self.set_clear_color_by_slot(clear_color, 0);
  }
  pub fn set_clear_color_by_slot(&mut self, clear_color: Vec4, slot: i32) {
    if slot < 0 || slot >= MAX_OUTPUT_SLOT as i32 {
      // TODO:
      // prpr::log::info()
      // prpr::log::warning()
      // prpr::log::error()
      // js::console::error()
      return;
    }
    self.clear_colors[slot as usize] = clear_color;
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
    gl.flush();
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
    Pipeline {
      gl: Rc::clone(&self.gl),
    }
  }
  pub fn new_renderpass(&self) -> RenderPass {
    RenderPass::new(Rc::clone(&self.gl))
  }
}
