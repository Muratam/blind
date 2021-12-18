use super::*;

pub struct Pipeline {
  gl: Rc<GlContext>,
  // states
  draw_command: Option<DrawCommand>,
  cull_mode: CullMode,
  primitive_topology: PrimitiveToporogy,
  shader: Option<Rc<Shader>>,
  descriptor: Descriptor,
}

impl Pipeline {
  pub fn new(gl: &Rc<GlContext>) -> Self {
    Self {
      gl: Rc::clone(gl),
      draw_command: None,
      cull_mode: CullMode::Back,
      primitive_topology: PrimitiveToporogy::Triangles,
      shader: None,
      descriptor: Descriptor::new(),
    }
  }

  pub fn draw(&mut self) {
    let gl = &self.gl;
    let mut outer_desc_ctx = DescriptorContext::Nil;
    if let Some(shader) = &self.shader {
      shader.use_program();
      outer_desc_ctx
        .cons(&mut self.descriptor)
        .bind(shader.raw_program());
    } else {
      log::error("No Shader Program");
      return;
    }
    if self.cull_mode == CullMode::None {
      self.gl.disable(gl::CULL_FACE);
    } else {
      self.gl.enable(gl::CULL_FACE);
      self.gl.cull_face(self.cull_mode as u32);
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
  // set resource
  pub fn set_shader(&mut self, shader: &Rc<Shader>) {
    self.shader = Some(Rc::clone(shader));
  }
  pub fn set_vao<T: BufferAttribute + 'static>(&mut self, vao: &VaoPtr<T>) {
    self.descriptor.set_vao(&(Rc::clone(vao) as VaoDynPtr));
  }
  pub fn set_draw_vao<T: BufferAttribute + 'static>(&mut self, vao: &VaoPtr<T>) {
    self.set_vao(vao);
    self.set_draw_command(vao.borrow().draw_command());
  }
  pub fn add_uniform_buffer<T: BufferAttribute + 'static>(&mut self, buffer: &UniformBufferPtr<T>) {
    self
      .descriptor
      .add_uniform_buffer(&(Rc::clone(buffer) as UniformBufferDynPtr));
  }
  pub fn set_cull_mode(&mut self, mode: CullMode) {
    self.cull_mode = mode;
  }
  // draw
  pub fn set_draw_command(&mut self, command: DrawCommand) {
    self.draw_command = Some(command);
  }
  pub fn set_draw_mode(&mut self, primitive_topology: PrimitiveToporogy) {
    self.primitive_topology = primitive_topology;
  }
}
