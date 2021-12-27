use super::*;

crate::shader_attr! {
  struct FullScreenVertex {
    position: vec2,
    uv: vec2,
  }
}
pub struct FullScreen {
  vao: Arc<Vao<FullScreenVertex>>,
}
impl FullScreen {
  pub fn new(ctx: &ArcGlContext) -> Self {
    // バインディングなしでも行けるがそんなに変わらないので
    let i_buffer = IndexBuffer::new(ctx, vec![0, 1, 2, 2, 1, 3]);
    let v_buffer = VertexBuffer::new(
      ctx,
      vec![
        FullScreenVertex {
          position: Vec2::new(-1.0, -1.0),
          uv: Vec2::new(0.0, 0.0),
        },
        FullScreenVertex {
          position: Vec2::new(1.0, -1.0),
          uv: Vec2::new(1.0, 0.0),
        },
        FullScreenVertex {
          position: Vec2::new(-1.0, 1.0),
          uv: Vec2::new(0.0, 1.0),
        },
        FullScreenVertex {
          position: Vec2::new(1.0, 1.0),
          uv: Vec2::new(1.0, 1.0),
        },
      ],
    );
    Self {
      vao: Arc::new(Vao::new(ctx, v_buffer, i_buffer)),
    }
  }
  pub fn new_pipeline(ctx: &ArcGlContext) -> Pipeline {
    let mut pipeline = Pipeline::new(ctx);
    pipeline.set_depth_func(DepthFunc::Always);
    pipeline.set_cull_mode(CullMode::None);
    pipeline.add(&Self::new(ctx));
    pipeline
  }
}
impl PipelineBindable for FullScreen {
  fn bind_pipeline(&self, pipeline: &mut Pipeline) {
    pipeline.set_draw_vao(&self.vao);
  }
}

crate::shader_attr! {
  struct ShapeVertex {
    position: vec3,
    normal: vec3,
    // tangent: vec3,
    // uv: vec2
  }
}
pub struct Shape {
  vao: Arc<Vao<ShapeVertex>>,
}
impl Shape {
  pub fn new_cube(ctx: &ArcGlContext) -> Self {
    let mut v_data: Vec<ShapeVertex> = Vec::new();
    for x in [-0.5, 0.5] {
      for y in [-0.5, 0.5] {
        for z in [-0.5, 0.5] {
          let position = Vec3::new(x, y, z);
          for normal in [
            x.signum() * Vec3::X,
            y.signum() * Vec3::Y,
            z.signum() * Vec3::Z,
          ] {
            v_data.push(ShapeVertex { position, normal })
          }
        }
      }
    }
    let i_data: Vec<IndexBufferType> = vec![
      0, 3, 6, 9, 6, 3, //
      15, 12, 18, 18, 21, 15, //
      1, 16, 4, 16, 1, 13, //
      7, 10, 22, 22, 19, 7, //
      2, 8, 14, 14, 8, 20, //
      5, 17, 11, 17, 23, 11, //
    ];
    let i_buffer = IndexBuffer::new(ctx, i_data);
    let v_buffer = VertexBuffer::new(ctx, v_data);
    Self {
      vao: Arc::new(Vao::new(ctx, v_buffer, i_buffer)),
    }
  }
}
impl PipelineBindable for Shape {
  fn bind_pipeline(&self, pipeline: &mut Pipeline) {
    pipeline.set_draw_vao(&self.vao);
  }
}

pub struct Mesh {}
impl Mesh {}
