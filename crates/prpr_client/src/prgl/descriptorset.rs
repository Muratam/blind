use super::*;
/*
- + DC0(=Cons(D0, Nil)) ...
  - RenderPass + DC1(=Cons(D1, DC0))
    - (PipelineA + DescriptorSet0A)
    - (PipelineA + DescriptorSet0B)
    - (PipelineB + DescriptorSet0B)
    - ...
    - ... ShaderBind時にデフォルトテクスチャを貼っておけばよろし？
*/
pub struct Descriptor {
  vao: Option<Box<dyn VaoTrait>>,
  u_buffers: Vec<Box<dyn UniformBufferTrait>>,
  // u_textures: Vec<Texture>
}
impl Descriptor {
  pub fn new(
    vao: Option<Box<dyn VaoTrait>>,
    u_buffers: Vec<Box<dyn UniformBufferTrait>>,
  ) -> Descriptor {
    Self { vao, u_buffers }
  }
}
pub enum DescriptorContext<'a, 'b> {
  Cons {
    prior: &'a mut Descriptor,
    others: &'b mut DescriptorContext<'a, 'b>,
  },
  Nil,
}

impl<'a, 'b> DescriptorContext<'a, 'b> {
  pub fn cons(&'a mut self, prior: &'b mut Descriptor) -> DescriptorContext<'a, 'b> {
    Self::Cons {
      prior,
      others: self,
    }
  }
  pub fn bind(&mut self, program: &RawShaderProgram) {
    if let Self::Cons { prior, others } = self {
      others.bind(program);
      for u_buffer in &mut prior.u_buffers {
        u_buffer.bind(program);
      }
      if let Some(vao) = &mut prior.vao {
        vao.bind(program);
      } else {
        log::error("No Vertex Array Object");
        return;
      }
    }
  }
}
