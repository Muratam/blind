use super::*;

pub struct Pipeline {
  gl: Rc<GlContext>,
  // not vao
  raw_vao: Option<RawVao>,
  // states
  raw_shader_program: Option<RawShaderProgram>,
  draw_command: Option<DrawCommand>,
  primitive_topology: PrimitiveToporogy,
}

// - 1 Pipeline は 1 ShaderProgram を持つ
// -
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
    crate::shader_attr! {
      struct Vertex {
        color: vec4,
        position: vec3,
      }
      struct Global {
        add_color: vec4,
      }
    }
    let template = crate::shader_template! {
      attrs: [Global],
      vs_attr: Vertex,
      fs_attr: { in_color: vec4 },
      out_attr: { out_color: vec4 }
      vs_code: {
        in_color = color;
        gl_Position = vec4(position, 1.0);
      },
      fs_code: {
        out_color = in_color + add_color;
      }
    };
    self.raw_shader_program = RawShaderProgram::new(Rc::clone(&self.gl), &template);
    if let Some(program) = &self.raw_shader_program {
      // buffer
      let v_data = vec![
        Vertex {
          position: Vec3::Y,
          color: Vec4::X + Vec4::W,
        },
        Vertex {
          position: Vec3::X,
          color: Vec4::Y + Vec4::W,
        },
        Vertex {
          position: -Vec3::X,
          color: Vec4::Z + Vec4::W,
        },
        Vertex {
          position: -Vec3::Y,
          color: Vec4::ONE,
        },
      ];
      let v_buffer = RawGpuBuffer::new(Rc::clone(&self.gl), v_data.as_slice(), BufferUsage::Vertex);
      let i_data: Vec<IndexBufferType> = vec![0, 1, 2, 2, 3, 1];
      let i_buffer = RawGpuBuffer::new(Rc::clone(&self.gl), i_data.as_slice(), BufferUsage::Index);
      self.raw_vao = Some(RawVao::new(
        Rc::clone(&self.gl),
        program.raw_program(),
        &template.vs_in_template(),
        &v_buffer,
        Some(&i_buffer),
      ));
      let u_data = Global {
        add_color: Vec4::new(0.5, 0.5, 0.5, 0.5),
      };
      let u_buffer = RawGpuBuffer::new(Rc::clone(&self.gl), u_data.ub_data(), BufferUsage::Uniform);
      let u_index = self
        .gl
        .get_uniform_block_index(&program.raw_program(), u_data.self_name());
      if u_index == gl::INVALID_INDEX {
        log::error(format!(
          "invalid uniform buffer name: {}",
          u_data.self_name()
        ));
      }
      self
        .gl
        .bind_buffer_base(gl::UNIFORM_BUFFER, u_index, Some(&u_buffer.raw_buffer()));
      self.set_draw_indexed(0, i_data.len() as i32);
    }
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
