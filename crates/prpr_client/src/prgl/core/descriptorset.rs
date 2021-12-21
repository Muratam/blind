use super::*;
// - DescriptorContext にどんどんconsしていって、適用する。
//   - Stackになるので、動作がわかりやすいはず
//   - RenderPass・Pipelineも持つ
// TODO: ShaderBind時にデフォルトテクスチャを貼る

pub struct Descriptor {
  vao: Option<Arc<dyn VaoTrait>>,
  u_buffers: Vec<Arc<dyn UniformBufferTrait>>,
  // u_textures: Vec<Texture>
}
impl Descriptor {
  pub fn new() -> Descriptor {
    Self {
      vao: None,
      u_buffers: Vec::new(),
    }
  }
  pub fn set_vao(&mut self, vao: &Arc<dyn VaoTrait>) {
    self.vao = Some(Arc::clone(vao));
  }
  pub fn add_uniform_buffer(&mut self, buffer: &Arc<dyn UniformBufferTrait>) {
    self.u_buffers.push(Arc::clone(buffer));
  }
}
pub enum DescriptorContext<'a, 'b> {
  Cons {
    prior: &'a Descriptor,
    others: &'b DescriptorContext<'a, 'b>,
  },
  Nil,
}

impl<'a, 'b> DescriptorContext<'a, 'b> {
  pub fn cons(&'a self, prior: &'b Descriptor) -> DescriptorContext<'a, 'b> {
    Self::Cons {
      prior,
      others: self,
    }
  }
  pub fn bind(&self, shader: &Shader) {
    if let Self::Cons { prior, others } = self {
      others.bind(shader);
      for u_buffer in &prior.u_buffers {
        u_buffer.bind(shader);
      }
      if let Some(vao) = &prior.vao {
        vao.bind(shader);
      } else {
        log::error("No Vertex Array Object");
        return;
      }
    }
  }
}
