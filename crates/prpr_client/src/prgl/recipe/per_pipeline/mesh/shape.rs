use super::*;

crate::shader_attr! {
  struct ShapeVertex {
    position: vec3,
    normal: vec3,
    // tangent: vec3,
    // uv: vec2
  }
}
pub struct Shape {
  vao: SOwner<Vao<ShapeVertex>>,
}
impl Shape {
  pub fn new_cube() -> Self {
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
    let i_buffer = IndexBuffer::new(i_data);
    let v_buffer = VertexBuffer::new(v_data);
    Self {
      vao: SOwner::new(Vao::new(v_buffer, i_buffer)),
    }
  }
  pub fn new_sphere(xn: usize, yn: usize) -> Self {
    use std::f32::consts::PI;
    let mut v_data: Vec<ShapeVertex> = Vec::new();
    for x in 0..=xn {
      let x = x as f32;
      let xn = xn as f32;
      let xrad = PI * 2.0 * x / xn;
      for y in 0..=yn {
        let y = y as f32;
        let yn = yn as f32;
        let yrad = PI * y / yn;
        let normal = Vec3::new(xrad.sin() * yrad.sin(), yrad.cos(), xrad.cos() * yrad.sin());
        let position = normal * 0.5;
        v_data.push(ShapeVertex { position, normal })
      }
    }
    let mut i_data: Vec<IndexBufferType> = Vec::new();
    for x in 1..=xn {
      for y in 1..=yn {
        let i = y + (yn + 1) * x;
        for idx in [
          i,
          i - 1,
          i - (yn + 1),
          i - (yn + 1),
          i - 1,
          i - (yn + 1) - 1,
        ] {
          i_data.push(idx as IndexBufferType);
        }
      }
    }
    let i_buffer = IndexBuffer::new(i_data);
    let v_buffer = VertexBuffer::new(v_data);
    Self {
      vao: SOwner::new(Vao::new(v_buffer, i_buffer)),
    }
  }
}
impl PipelineBindable for Shape {
  fn bind_pipeline(&self, pipeline: &mut Pipeline) {
    pipeline.set_draw_vao(&self.vao);
  }
}
