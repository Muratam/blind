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
enum DrawCommand {
  Arrays(i32, i32), // first, count
}
pub enum DrawMode {
  Points = gl::POINTS as isize,
  LineStrip = gl::LINE_STRIP as isize,
  LineLoop = gl::LINE_LOOP as isize,
  Lines = gl::LINES as isize,
  TriangleStrip = gl::TRIANGLE_STRIP as isize,
  TriangleFan = gl::TRIANGLE_FAN as isize,
  Triangles = gl::TRIANGLES as isize,
}
// struct RawPipeline {
//   is_enabled_depth_test: bool,
//   draw_command: RawDrawCommand,
//   // gl.depth_func(gl::LEQUAL);
// }

pub struct Pipeline {
  gl: Rc<WebGlContext>,
}
impl Pipeline {
  pub fn new(gl: Rc<WebGlContext>) -> Pipeline {
    Pipeline { gl: Rc::clone(&gl) }
  }
  pub fn draw(&self) {}
}
