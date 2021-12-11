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

pub struct DrawArrays {
  pub first: i32,
  pub count: i32,
}
pub enum DrawCommand {
  Arrays(DrawArrays),
  // ArraysInstanced(u32, u32, u32), // first, count, instance_count
  // Elements(u32, u32),             // count, (type), offset
  // ElementsInstanced(u32, u32, u32), // count, (type), offset, instance_count
  // Buffers([buf])
  // RangeElements(u32, u32, u32, u32) // start, end, count, (type), offset
}
#[derive(Clone, Copy)]
pub enum DrawMode {
  Points = gl::POINTS as isize,
  LineStrip = gl::LINE_STRIP as isize,
  LineLoop = gl::LINE_LOOP as isize,
  Lines = gl::LINES as isize,
  TriangleStrip = gl::TRIANGLE_STRIP as isize,
  TriangleFan = gl::TRIANGLE_FAN as isize,
  Triangles = gl::TRIANGLES as isize,
}

pub struct Pipeline {
  gl: Rc<WebGlContext>,
  draw_command: Option<DrawCommand>,
  draw_mode: DrawMode,
  //   is_enabled_depth_test: bool,
  // gl.depth_func(gl::LEQUAL);
  raw_shader_program: Option<RawShaderProgram>,
}
impl Pipeline {
  pub fn new(gl: Rc<WebGlContext>) -> Self {
    Self {
      gl: Rc::clone(&gl),
      draw_command: None,
      draw_mode: DrawMode::Triangles,
      raw_shader_program: None,
    }
  }
  pub fn setup_sample(&mut self) {
    let gl = &self.gl;
    let vs_code = "#version 300 es
    layout(location = 0) in vec3 position;
    // centroid out for msaa / smooth / flat /
    void main() { gl_Position = vec4(position, 1.0); }
    ";
    let fs_code = "#version 300 es
    precision highp float;
    out vec4 out_color;
    void main() { out_color = vec4(1.0f, 1.0f, 1.0f, 1.0f); }
    ";
    if let Some(vs_shader) = RawShader::new(gl.as_ref(), vs_code, ShaderType::VertexShader) {
      if let Some(fs_shader) = RawShader::new(gl.as_ref(), fs_code, ShaderType::FragmentShader) {
        self.raw_shader_program = RawShaderProgram::new(gl.as_ref(), &vec![vs_shader, fs_shader]);
      }
    }
    // create vbo
    self.set_draw_arrays(0, 0);
  }
  pub fn draw(&self) {
    if let Some(program) = &self.raw_shader_program {
      program.use_program(self.gl.as_ref());
    } else {
      log::error("No Shader Program");
      return;
    }
    let draw_mode = self.draw_mode as u32;
    if let Some(command) = &self.draw_command {
      match &command {
        DrawCommand::Arrays(command) => {
          self.gl.draw_arrays(draw_mode, command.first, command.count);
        }
      }
    } else {
      log::error("No Draw Command");
      return;
    }
  }
  pub fn set_draw_arrays(&mut self, first: i32, count: i32) {
    self.draw_command = Some(DrawCommand::Arrays(DrawArrays {
      first: first,
      count: count,
    }));
  }
  pub fn set_draw_mode(&mut self, mode: DrawMode) {
    self.draw_mode = mode;
  }
}
