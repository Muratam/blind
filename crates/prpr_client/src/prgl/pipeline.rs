use super::*;

/*
- Instance
  - RenderPass
    - &Texture
  - Pipeline
    - ShaderProgram
  - DescriptorSet
    - &Texture
    - &Buffer (bind by name)
*/

pub struct Pipeline {
  gl: Rc<GlContext>,
  // states
  draw_command: Option<DrawCommand>,
  primitive_topology: PrimitiveToporogy,
  shader: Option<Shader>,
  descriptor: Option<Descriptor>,
}

impl Pipeline {
  pub fn new(gl: &Rc<GlContext>) -> Self {
    Self {
      gl: Rc::clone(gl),
      draw_command: None,
      primitive_topology: PrimitiveToporogy::Triangles,
      shader: None,
      descriptor: None,
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
    let i_data = vec![0, 1, 2, 2, 3, 1];
    let u_data = Global {
      add_color: Vec4::new(0.5, 0.5, 0.5, 0.5),
    };
    let i_size = i_data.len() as i32;
    let i_buffer = IndexBuffer::new(&self.gl, i_data);
    let v_buffer = VertexBuffer::new(&self.gl, v_data);
    let vao = Vao::new(&self.gl, v_buffer, i_buffer);
    let u_buffer = UniformBuffer::new(&self.gl, u_data);
    self.descriptor = Some(Descriptor::new(
      Some(Box::new(vao)),
      vec![Box::new(u_buffer)],
    ));
    self.shader = Shader::new(&self.gl, template);
    self.set_draw_indexed(0, i_size);
  }

  pub fn draw(&mut self) {
    let gl = &self.gl;
    let mut outer_desc_ctx = DescriptorContext::Nil;
    if let Some(shader) = &self.shader {
      shader.use_program();
      if let Some(descriptor) = &mut self.descriptor {
        outer_desc_ctx.cons(descriptor).bind(shader.raw_program());
      } else {
        outer_desc_ctx.bind(shader.raw_program());
      }
    } else {
      log::error("No Shader Program");
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
