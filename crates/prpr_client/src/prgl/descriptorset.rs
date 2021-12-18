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
  gl: Rc<GlContext>,
  // vao: Option<Vao>,
  // u_buffers: Vec<UniformBuffer>,
  // u_textures: Vec<Texture>
}
pub enum DescriptorContext {
  Cons {
    prior: Descriptor,
    tail: Box<DescriptorContext>,
  },
  Nil,
}
impl DescriptorContext {
  pub fn apply(&self) {
    // match self {
    //   Self::Cons(_, _) => {}
    //   Self::Nil => {}
    // }
  }
}
