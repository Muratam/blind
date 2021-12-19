use super::*;

pub struct Pipeline {
  gl: Arc<GlContext>,
  // states
  draw_command: Option<DrawCommand>,
  cull_mode: CullMode,
  primitive_topology: PrimitiveToporogy,
  shader: Option<Arc<Shader>>,
  descriptor: Descriptor,
}

impl Pipeline {
  pub fn new(gl: &Arc<GlContext>) -> Self {
    Self {
      gl: Arc::clone(gl),
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
    self.cull_mode.apply(&self.gl);
    if let Some(draw_command) = &self.draw_command {
      draw_command.apply(&self.gl, self.primitive_topology);
    } else {
      log::error("No Draw Command");
      return;
    }
  }
  // set resource
  pub fn set_shader(&mut self, shader: &Arc<Shader>) {
    self.shader = Some(Arc::clone(shader));
  }
  pub fn set_vao<T: BufferAttribute + 'static>(&mut self, vao: &VaoPtr<T>) {
    self.descriptor.set_vao(&(Arc::clone(vao) as VaoDynPtr));
  }
  pub fn set_draw_vao<T: BufferAttribute + 'static>(&mut self, vao: &VaoPtr<T>) {
    self.set_vao(vao);
    self.set_draw_command(vao.read().unwrap().draw_command());
  }
  pub fn add_uniform_buffer<T: BufferAttribute + 'static>(&mut self, buffer: &UniformBufferPtr<T>) {
    self
      .descriptor
      .add_uniform_buffer(&(Arc::clone(buffer) as UniformBufferDynPtr));
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
