use super::*;

crate::shader_attr! {
  struct FullScreenVertex {
    position: vec2,
  }
}
pub struct FullScreen {
  vao: SOwner<Vao<FullScreenVertex>>,
}
impl FullScreen {
  pub fn new() -> Self {
    // バインディングなしでも行けるがそんなに変わらないので
    let i_buffer = IndexBuffer::new(vec![0, 1, 2, 2, 1, 3]);
    let v_buffer = VertexBuffer::new(vec![
      FullScreenVertex {
        position: Vec2::new(-1.0, -1.0),
      },
      FullScreenVertex {
        position: Vec2::new(1.0, -1.0),
      },
      FullScreenVertex {
        position: Vec2::new(-1.0, 1.0),
      },
      FullScreenVertex {
        position: Vec2::new(1.0, 1.0),
      },
    ]);
    Self {
      vao: SOwner::new(Vao::new(v_buffer, i_buffer)),
    }
  }
  pub fn new_pipeline() -> Pipeline {
    let mut pipeline = Pipeline::new();
    pipeline.set_depth_func(DepthFunc::Always);
    pipeline.set_cull_mode(CullMode::None);
    pipeline.add(&Self::new());
    pipeline
  }
}
impl PipelineBindable for FullScreen {
  fn bind_pipeline(&self, pipeline: &mut Pipeline) {
    pipeline.set_draw_vao(&self.vao);
  }
}
