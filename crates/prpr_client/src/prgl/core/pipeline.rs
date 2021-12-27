use super::*;

pub struct Pipeline {
  ctx: ArcGlContext,
  // states
  depth_func: DepthFunc,
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
      depth_func: DepthFunc::Less,
      draw_command: None,
      cull_mode: CullMode::Back,
      primitive_topology: PrimitiveToporogy::Triangles,
      shader: None,
      descriptor: Descriptor::new(),
    }
  }

  pub fn draw(&self, cmd: &mut Command, outer_desc_ctx: &DescriptorContext) {
    if let Some(shader) = &self.shader {
      cmd.set_shader(shader);
      outer_desc_ctx.cons(&self.descriptor).bind(cmd);
    } else {
      log::error("No Shader Program");
      return;
    }
    cmd.set_depth_func(self.depth_func);
    cmd.set_cull_mode(self.cull_mode);
    if let Some(draw_command) = &self.draw_command {
      cmd.set_draw_command(draw_command, self.primitive_topology);
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
  pub fn add_uniform_buffer_trait(&mut self, buffer: &Arc<dyn UniformBufferTrait>) {
    self.descriptor.add_uniform_buffer(&buffer.clone());
  }
  pub fn add_uniform_buffer<T: BufferAttribute + 'static>(
    &mut self,
    buffer: &Arc<UniformBuffer<T>>,
  ) {
    self.add_uniform_buffer_trait(&(buffer.clone() as Arc<dyn UniformBufferTrait>));
  }
  pub fn add_into_uniform_buffer<T: BufferAttribute + 'static, I: RefInto<T> + 'static>(
    &mut self,
    buffer: &Arc<IntoUniformBuffer<T, I>>,
  ) {
    self.add_uniform_buffer_trait(&(buffer.clone() as Arc<dyn UniformBufferTrait>));
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

pub trait PipelineBindable {
  fn bind_pipeline(&self, pipeline: &mut Pipeline);
}
