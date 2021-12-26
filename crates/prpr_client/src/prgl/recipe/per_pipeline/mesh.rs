use super::*;

// vbo per pipeline

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
  fn bind(&self, pipeline: &mut Pipeline) {
    pipeline.set_draw_vao(&self.vao);
  }
}

pub struct Mesh {}
impl Mesh {}
