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
  ) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self { vao, u_buffers }))
  }
}
pub enum DescriptorContext {
  Cons {
    prior: Rc<RefCell<Descriptor>>,
    others: Rc<RefCell<DescriptorContext>>,
  },
  Nil,
}
impl DescriptorContext {
  pub fn cons(
    prior: &Rc<RefCell<Descriptor>>,
    others: &Rc<RefCell<DescriptorContext>>,
  ) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self::Cons {
      prior: Rc::clone(prior),
      others: Rc::clone(others),
    }))
  }
  pub fn bind(&mut self, program: &RawShaderProgram) {
    if let Self::Cons { prior, others } = self {
      others.borrow_mut().bind(program);
      for u_buffer in &mut prior.borrow_mut().u_buffers {
        u_buffer.bind(program);
      }
      if let Some(vao) = &mut prior.borrow_mut().vao {
        vao.bind(program);
      } else {
        log::error("No Vertex Array Object");
        return;
      }
    }
  }
}
