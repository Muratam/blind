use super::*;
// 動作イメージ
// - + DC0(=Cons(D0, Nil)) ...
//   - RenderPass + DC1(=Cons(D1, DC0))
//     - (PipelineA + DescriptorSet0A)
//     - (PipelineA + DescriptorSet0B)
//     - (PipelineB + DescriptorSet0B)
//     - ...
//     - ... ShaderBind時にデフォルトテクスチャを貼っておけばよろし？

// ワールド側での更新と描画側での更新が同時に発生する可能性があるため、
// Rc<RefCell<T>> が必要。panicに応じて排他を入れればOK
pub type UniformBufferDynPtr = Rc<RefCell<dyn UniformBufferTrait>>;
pub type UniformBufferPtr<T> = Rc<RefCell<UniformBuffer<T>>>;
pub type VaoDynPtr = Rc<RefCell<dyn VaoTrait>>;
pub type VaoPtr<T> = Rc<RefCell<Vao<T>>>;

pub struct Descriptor {
  vao: Option<VaoDynPtr>,
  u_buffers: Vec<UniformBufferDynPtr>,
  // u_textures: Vec<Texture>
}
impl Descriptor {
  pub fn new() -> Descriptor {
    Self {
      vao: None,
      u_buffers: Vec::new(),
    }
  }
  pub fn set_vao(&mut self, vao: &VaoDynPtr) {
    self.vao = Some(Rc::clone(vao));
  }
  pub fn add_uniform_buffer(&mut self, buffer: &UniformBufferDynPtr) {
    self.u_buffers.push(Rc::clone(buffer));
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
        u_buffer.borrow_mut().bind(program);
      }
      if let Some(vao) = &mut prior.vao {
        vao.borrow_mut().bind(program);
      } else {
        log::error("No Vertex Array Object");
        return;
      }
    }
  }
}
