use super::*;

pub struct Pipeline {
  // states
  depth_func: DepthFunc,
  draw_command: Option<DrawCommand>,
  cull_mode: CullMode,
  primitive_topology: PrimitiveToporogy,
  shader: Option<Arc<Shader>>,
  invisible_reasons: collections::BitSet64,
  descriptor: Primary<Descriptor>,
}

impl Pipeline {
  pub fn new() -> Self {
    Self {
      depth_func: DepthFunc::Less,
      draw_command: None,
      cull_mode: CullMode::Back,
      primitive_topology: PrimitiveToporogy::Triangles,
      shader: None,
      invisible_reasons: collections::BitSet64::new(),
      descriptor: Primary::new(Descriptor::new()),
    }
  }

  pub fn draw(&self, cmd: &mut Command, outer_ctx: &Arc<DescriptorContext>) {
    if self.invisible() {
      return;
    }
    if let Some(shader) = &self.shader {
      cmd.set_shader(shader);
      DescriptorContext::cons(outer_ctx, &self.descriptor).bind(cmd);
    } else {
      // log::error("No Shader Program");
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
  pub fn set_vao<T: BufferAttribute + 'static>(&mut self, vao: &dyn ReplicaTrait<Vao<T>>) {
    let mut descriptor = self.descriptor.write();
    descriptor.set_vao(Box::new(vao.clone_replica()) as Box<dyn VaoTrait>);
  }
  pub fn set_draw_vao<T: BufferAttribute + 'static>(&mut self, vao: &dyn ReplicaTrait<Vao<T>>) {
    self.set_vao(vao);
    self.set_draw_command(vao.read().draw_command());
  }
  pub fn add_uniform_buffer_trait(&mut self, buffer: Box<dyn UniformBufferTrait>) {
    let mut descriptor = self.descriptor.write();
    descriptor.add_uniform_buffer(buffer);
  }
  pub fn add_uniform_buffer<T: BufferAttribute + 'static>(
    &mut self,
    buffer: &dyn ReplicaTrait<UniformBuffer<T>>,
  ) {
    self.add_uniform_buffer_trait(Box::new(buffer.clone_replica()) as Box<dyn UniformBufferTrait>);
  }
  pub fn add_uniform_buffer_reader<T: BufferAttribute + 'static>(
    &mut self,
    buffer: &Replica<UniformBuffer<T>>,
  ) {
    self.add_uniform_buffer(buffer);
  }

  pub fn add_into_uniform_buffer<T: BufferAttribute + 'static, I: RefInto<T> + 'static>(
    &mut self,
    buffer: &dyn ReplicaTrait<IntoUniformBuffer<T, I>>,
  ) {
    self.add_uniform_buffer_trait(Box::new(buffer.clone_replica()) as Box<dyn UniformBufferTrait>);
  }
  pub fn add_into_uniform_buffer_reader<T: BufferAttribute + 'static, I: RefInto<T> + 'static>(
    &mut self,
    buffer: &Replica<IntoUniformBuffer<T, I>>,
  ) {
    self.add_into_uniform_buffer(buffer);
  }
  pub fn add_texture_mapping<T: TextureMappingAttribute + 'static>(
    &mut self,
    mapping: &dyn ReplicaTrait<TextureMapping<T>>,
  ) {
    let mut descriptor = self.descriptor.write();
    descriptor
      .add_texture_mapping(Box::new(mapping.clone_replica()) as Box<dyn TextureMappingTrait>);
  }
  pub fn add_texture_mapping_reader<T: TextureMappingAttribute + 'static>(
    &mut self,
    mapping: &Replica<TextureMapping<T>>,
  ) {
    self.add_texture_mapping(mapping);
  }
  pub fn set_cull_mode(&mut self, mode: CullMode) {
    self.cull_mode = mode;
  }
  // draw
  pub fn set_draw_command(&mut self, command: DrawCommand) {
    self.draw_command = Some(command);
  }
  pub fn set_depth_func(&mut self, depth_func: DepthFunc) {
    self.depth_func = depth_func;
  }
  pub fn set_draw_mode(&mut self, primitive_topology: PrimitiveToporogy) {
    self.primitive_topology = primitive_topology;
  }
  pub fn set_invisible(&mut self, invisible: bool, reason: usize) {
    self.invisible_reasons.set(reason, invisible);
  }
  pub fn invisible(&self) -> bool {
    self.invisible_reasons.any()
  }
  pub fn add(&mut self, bindable: &dyn PipelineBindable) {
    bindable.bind_pipeline(self);
  }
}

pub trait PipelineBindable {
  fn bind_pipeline(&self, pipeline: &mut Pipeline);
}
