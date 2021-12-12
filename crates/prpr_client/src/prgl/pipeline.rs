use super::*;

enum DrawCommand {
  // draw
  Arrays { first: i32, count: i32 },
  // ArraysInstanced {
  //   first: i32,
  //   count: i32,
  //   instance_count: i32,
  // },
  // draw_indexed
  // Elements(u32, u32),             // count, (type), offset
  // ElementsInstanced(u32, u32, u32), // count, (type), offset, instance_count
  // Buffers([buf])
  // RangeElements(u32, u32, u32, u32) // start, end, count, (type), offset
}

#[derive(Clone, Copy)]
pub enum PrimitiveToporogy {
  Points = gl::POINTS as isize,
  LineStrip = gl::LINE_STRIP as isize,
  LineLoop = gl::LINE_LOOP as isize,
  Lines = gl::LINES as isize,
  TriangleStrip = gl::TRIANGLE_STRIP as isize,
  TriangleFan = gl::TRIANGLE_FAN as isize,
  Triangles = gl::TRIANGLES as isize,
}

pub struct Pipeline {
  gl: Rc<GlContext>,
  draw_command: Option<DrawCommand>,
  primitive_topology: PrimitiveToporogy,
  raw_shader_program: Option<RawShaderProgram>,
  // BlendState
  // ColorState
  // CullState
  // DepthState
  // CoverageState
  // PolygonOffsetState
  // StencilState
  // Scissor
  // Viewport
}
impl Pipeline {
  pub fn new(gl: Rc<GlContext>) -> Self {
    Self {
      gl: Rc::clone(&gl),
      draw_command: None,
      primitive_topology: PrimitiveToporogy::Triangles,
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
    {
      // vertex buffer
      // log::debug(std::mem::size_of::<VertexType>());
      type VertexType = Vec3;
      let data = vec![Vec3::Y, Vec3::X, -1.0 * Vec3::X];
      let location = 0;
      let attr_type = gl::FLOAT;
      let stride = 3;
      //
      let count = data.len();
      let buffer = RawGpuBuffer::new::<VertexType>(gl.as_ref(), count, BufferUsage::Vertex);
      buffer.write::<VertexType>(gl.as_ref(), 0, data.as_slice());
      gl.bind_buffer(buffer.raw_target(), Some(buffer.raw_buffer()));
      gl.enable_vertex_attrib_array(location);
      gl.vertex_attrib_pointer_with_i32(location, stride, attr_type, false, 0, 0);
      gl.bind_buffer(buffer.raw_target(), None);
    }
    self.set_draw(0, 3);
  }
  pub fn draw(&self) {
    if let Some(program) = &self.raw_shader_program {
      self.gl.use_program(Some(program.raw_program()));
    } else {
      log::error("No Shader Program");
      return;
    }
    let topology = self.primitive_topology as u32;
    if let Some(command) = &self.draw_command {
      match &command {
        DrawCommand::Arrays { first, count } => {
          self.gl.draw_arrays(topology, *first, *count);
        }
      }
    } else {
      log::error("No Draw Command");
      return;
    }
  }
  pub fn set_draw(&mut self, first: i32, count: i32) {
    self.draw_command = Some(DrawCommand::Arrays { first, count });
  }
  pub fn set_draw_mode(&mut self, primitive_topology: PrimitiveToporogy) {
    self.primitive_topology = primitive_topology;
  }
}
