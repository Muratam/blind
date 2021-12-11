use super::*;

// // 実態として必要な情報を全て詰め込んだもの
// pub const SWAP_COUNT: i64 = 2;
// struct RawBuffer {
//   need_swap: bool,
// }
// struct RawTexture {
//   need_swap: bool,
// }
// struct RawSampler {}
// struct RawExtensions {}
// struct RawShader {}
mod draw {
  use super::*;
  pub struct Arrays {
    pub first: u32,
    pub count: u32,
  }
  pub enum Command {
    Arrays(Arrays),
    // ArraysInstanced(u32, u32, u32), // first, count, instance_count
    // Elements(u32, u32),             // count, (type), offset
    // ElementsInstanced(u32, u32, u32), // count, (type), offset, instance_count
    // Buffers([buf])
    // RangeElements(u32, u32, u32, u32) // start, end, count, (type), offset
  }
  pub enum Mode {
    Points = gl::POINTS as isize,
    LineStrip = gl::LINE_STRIP as isize,
    LineLoop = gl::LINE_LOOP as isize,
    Lines = gl::LINES as isize,
    TriangleStrip = gl::TRIANGLE_STRIP as isize,
    TriangleFan = gl::TRIANGLE_FAN as isize,
    Triangles = gl::TRIANGLES as isize,
  }
}
// struct RawPipeline {
//   is_enabled_depth_test: bool,
//   // gl.depth_func(gl::LEQUAL);
// }

pub use draw::Mode as DrawMode;
pub struct Pipeline {
  gl: Rc<WebGlContext>,
  draw_command: Option<draw::Command>,
  draw_mode: draw::Mode,
}
impl Pipeline {
  pub fn new(gl: Rc<WebGlContext>) -> Pipeline {
    Pipeline {
      gl: Rc::clone(&gl),
      draw_command: None,
      draw_mode: draw::Mode::Triangles,
    }
  }
  pub fn setup_sample(&mut self) {
    // create vs
    // create fs
    // combine
    // create vbo
    self.set_draw_arrays(0, 0);
  }
  pub fn draw(&self) {
    if self.draw_command.is_none() {
      log::error("No Draw Command");
      return;
    }
  }
  pub fn set_draw_arrays(&mut self, first: u32, count: u32) {
    self.draw_command = Some(draw::Command::Arrays(draw::Arrays {
      first: first,
      count: count,
    }));
  }
  pub fn set_draw_mode(&mut self, mode: DrawMode) {
    self.draw_mode = mode;
  }
}
