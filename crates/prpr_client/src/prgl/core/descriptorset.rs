use super::*;
// - DescriptorContext にどんどんconsしていって、適用する。
//   - Stackになるので、動作がわかりやすいはず
//   - RenderPass・Pipelineも持つ
// TODO: ShaderBind時にデフォルトテクスチャを貼る

pub struct Descriptor {
  vao: Option<Box<dyn VaoTrait>>,
  u_buffers: Vec<Box<dyn UniformBufferTrait>>,
  u_mappings: Vec<Box<dyn TextureMappingTrait>>,
}
impl Descriptor {
  pub fn new() -> Descriptor {
    Self {
      vao: None,
      u_buffers: Vec::new(),
      u_mappings: Vec::new(),
    }
  }
  pub fn set_vao(&mut self, vao: Box<dyn VaoTrait>) {
    self.vao = Some(vao);
  }
  pub fn add_uniform_buffer(&mut self, buffer: Box<dyn UniformBufferTrait>) {
    self.u_buffers.push(buffer);
  }
  pub fn add_texture_mapping(&mut self, mapping: Box<dyn TextureMappingTrait>) {
    self.u_mappings.push(mapping);
  }
}
pub enum DescriptorContext {
  Cons {
    prior: ArcReader<Descriptor>,
    others: Arc<Self>,
  },
  Nil,
}

impl DescriptorContext {
  pub fn nil() -> Arc<Self> {
    Arc::new(Self::Nil)
  }
  pub fn cons(others: &Arc<Self>, prior: &dyn ReplicaTrait<Descriptor>) -> Arc<Self> {
    Arc::new(Self::Cons {
      prior: prior.clone_reader(),
      others: others.clone(),
    })
  }
  pub fn bind(&self, cmd: &mut Command) {
    if let Self::Cons { prior, others } = self {
      others.bind(cmd);
      let prior = prior.read();
      for u_buffer in &prior.u_buffers {
        u_buffer.bind(cmd);
      }
      for u_mapping in &prior.u_mappings {
        u_mapping.bind(cmd);
      }
      if let Some(vao) = &prior.vao {
        vao.bind(cmd);
      }
    }
  }
}
