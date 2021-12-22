use super::*;

pub struct Pipeline {
  ctx: ArcGlContext,
  // states
  draw_command: Option<DrawCommand>,
  cull_mode: CullMode,
  primitive_topology: PrimitiveToporogy,
  shader: Option<Arc<Shader>>,
  descriptor: Descriptor,
}

impl Pipeline {
  pub fn new(ctx: &ArcGlContext) -> Self {
    Self {
      ctx: ctx.clone(),
      draw_command: None,
      cull_mode: CullMode::Back,
      primitive_topology: PrimitiveToporogy::Triangles,
      shader: None,
      descriptor: Descriptor::new(),
    }
  }

  pub fn draw(&self) {
    let mut outer_desc_ctx = DescriptorContext::Nil;
    if let Some(shader) = &self.shader {
      shader.use_program();
      outer_desc_ctx.cons(&self.descriptor).bind(shader);
    } else {
      log::error("No Shader Program");
      return;
    }
    self.cull_mode.apply(&self.ctx);
    if let Some(draw_command) = &self.draw_command {
      draw_command.apply(&self.ctx, self.primitive_topology);
    } else {
      log::error("No Draw Command");
      return;
    }
  }
  // set resource
  pub fn set_shader(&mut self, shader: &Arc<Shader>) {
    self.shader = Some(Arc::clone(shader));
  }
  pub fn set_vao<T: BufferAttribute + 'static>(&mut self, vao: &Arc<Vao<T>>) {
    self
      .descriptor
      .set_vao(&(Arc::clone(vao) as Arc<dyn VaoTrait>));
  }
  pub fn set_draw_vao<T: BufferAttribute + 'static>(&mut self, vao: &Arc<Vao<T>>) {
    self.set_vao(vao);
    self.set_draw_command(vao.draw_command());
  }
  pub fn add_uniform_buffer<T: BufferAttribute + 'static>(
    &mut self,
    buffer: &Arc<UniformBuffer<T>>,
  ) {
    self
      .descriptor
      .add_uniform_buffer(&(Arc::clone(buffer) as Arc<dyn UniformBufferTrait>));
  }
  pub fn add_texture_mapping<T: TextureMappingAttribute + 'static>(
    &mut self,
    mapping: &Arc<TextureMapping<T>>,
  ) {
    self
      .descriptor
      .add_texture_mapping(&(Arc::clone(mapping) as Arc<dyn TextureMappingTrait>));
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
