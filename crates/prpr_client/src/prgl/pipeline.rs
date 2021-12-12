use super::*;

enum DrawCommand {
  Draw { first: i32, count: i32 },
  // DrawInstanced {
  //   first: i32,
  //   count: i32,
  //   instance_count: i32,
  // },
  DrawIndexed { first: i32, count: i32 },
  // DrawIndexedInstanced {
  //   first: i32,
  //   count: i32,
  //   instance_count: i32,
  // },
  // Buffers([buf])
  // RangeElements { u32, u32, u32, u32 }  // start, end, count, (type), offset
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
  raw_vao: Option<RawVao>,
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
      raw_vao: None,
    }
  }
  pub fn setup_sample(&mut self) {
    /*
    vertex_attribute! {
      position: vec3,
      color: vec4
    }
    -> Vec<RawVertexAttribute>,
      "layout(location = 0) in vec3 vs_in_position;
       layout(location = 1) in vec4 vs_in_color;"
    */
    let gl = self.gl.as_ref();
    #[repr(C)]
    struct VertexType {
      position: Vec3Attr,
      color: Vec4Attr,
    }
    let v_attrs = vec![
      RawVertexAttribute {
        name: String::from("position"),
        primitive_type: RawVertexAttributePrimitiveType::f32,
        count: 3,
        location: 0,
      },
      RawVertexAttribute {
        name: String::from("color"),
        primitive_type: RawVertexAttributePrimitiveType::f32,
        count: 4,
        location: 1,
      },
    ];
    let v_data = vec![
      VertexType {
        position: Vec3Attr::new(Vec3::Y),
        color: Vec4Attr::new(Vec4::X + Vec4::W),
      },
      VertexType {
        position: Vec3Attr::new(Vec3::X),
        color: Vec4Attr::new(Vec4::ZERO),
      },
      VertexType {
        position: Vec3Attr::new(-Vec3::X),
        color: Vec4Attr::new(Vec4::ZERO),
      },
      VertexType {
        position: Vec3Attr::new(-Vec3::Y),
        color: Vec4Attr::new(Vec4::X + Vec4::W),
      },
    ];
    let i_data: Vec<IndexBufferType> = vec![0, 1, 2, 2, 3, 1];
    let common_header = "#version 300 es\nprecision highp float;";
    let vs_in_layouts = "
      layout(location = 0) in vec3 vs_in_position;
      layout(location = 1) in vec4 vs_in_color;
    ";
    let vs_code = format!(
      "{common_header}\n{vs_in_layouts}\n{body}",
      common_header = common_header,
      vs_in_layouts = vs_in_layouts,
      body = "
      out vec4 fs_in_color;
      // centroid out for msaa / smooth / flat /
      void main() {
        fs_in_color = vs_in_color;
        gl_Position = vec4(vs_in_position, 1.0);
      }
      "
    );
    let fs_code = format!(
      "{common_header}\n{body}",
      common_header = common_header,
      body = "
      in vec4 fs_in_color;
      out vec4 out_color;
      void main() { out_color = fs_in_color; }
      "
    );

    // shader
    let vertex_shader = RawShader::new(gl, &vs_code, ShaderType::VertexShader);
    let fragment_shader = RawShader::new(gl, &fs_code, ShaderType::FragmentShader);
    self.raw_shader_program = RawShaderProgram::new(
      gl,
      &RawShaderProgramContents {
        vertex_shader,
        fragment_shader,
      },
    );
    let v_buffer = RawGpuBuffer::new(gl, v_data.as_slice(), BufferUsage::Vertex);
    let i_buffer = RawGpuBuffer::new(gl, i_data.as_slice(), BufferUsage::Index);
    self.raw_vao = Some(RawVao::new(gl, &v_attrs, &v_buffer, Some(&i_buffer)));
    self.set_draw_indexed(0, 6);
  }
  pub fn draw(&self) {
    let gl = &self.gl;
    if let Some(program) = &self.raw_shader_program {
      gl.use_program(Some(program.raw_program()));
    } else {
      log::error("No Shader Program");
      return;
    }
    if let Some(vao) = &self.raw_vao {
      gl.bind_vertex_array(Some(vao.get_raw_vao()));
    } else {
      log::error("No Vertex Array Object");
      return;
    }
    let topology = self.primitive_topology as u32;
    if let Some(command) = &self.draw_command {
      match &command {
        DrawCommand::Draw { first, count } => {
          gl.draw_arrays(topology, *first, *count);
        }
        DrawCommand::DrawIndexed { first, count } => {
          assert_type_eq!(u32, IndexBufferType);
          gl.draw_elements_with_i32(topology, *count, gl::UNSIGNED_INT, *first);
        }
      }
    } else {
      log::error("No Draw Command");
      return;
    }
  }
  pub fn set_draw(&mut self, first: i32, count: i32) {
    self.draw_command = Some(DrawCommand::Draw { first, count });
  }
  pub fn set_draw_indexed(&mut self, first: i32, count: i32) {
    self.draw_command = Some(DrawCommand::DrawIndexed { first, count });
  }
  pub fn set_draw_mode(&mut self, primitive_topology: PrimitiveToporogy) {
    self.primitive_topology = primitive_topology;
  }
}
