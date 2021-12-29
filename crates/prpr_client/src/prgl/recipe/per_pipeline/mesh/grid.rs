use super::*;
/*
crate::shader_attr! {
  struct GridVertex {
    position: vec3,
  }
}
pub struct Grid {
  vao: ArcOwner<Vao<FullScreenVertex>>,
}
impl Grid {
  pub fn new(x: f32, y: f32, z: f32, interval: f32) -> Self {
    let mut interval = interval;
    if interval < 0.0 {
      log::error("grid interval < 0");
      interval = 1.0;
    }
    fn refine(a: f32) -> f32 {
      let eps = 0.001;
      let mut a = a.abs();
      if a < eps {
        a = eps;
      }
      a
    }
    let x = refine(x);
    let y = refine(y);
    let z = refine(z);
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
      vao: ArcOwner::new(Vao::new(v_buffer, i_buffer)),
    }
  }
  pub fn new_pipeline() -> Pipeline {
    let mut pipeline = Pipeline::new();
    pipeline.set_depth_func(DepthFunc::Always);
    pipeline.set_cull_mode(CullMode::None);
    // pipeline.add(&Self::new());
    pipeline
  }
}
impl PipelineBindable for Grid {
  fn bind_pipeline(&self, pipeline: &mut Pipeline) {
    pipeline.set_draw_vao(&self.vao);
  }
}
*/
