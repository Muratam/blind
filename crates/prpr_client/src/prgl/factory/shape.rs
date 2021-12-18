use super::*;

pub struct ShapeFactory {
  gl: Rc<GlContext>,
}
crate::shader_attr! {
  struct ShapeFactoryVertex {
    position: vec3,
    normal: vec3,
    // tangent: vec3,
    // uv: vec2
  }
}
impl ShapeFactory {
  pub fn new(gl: &Rc<GlContext>) -> Self {
    Self { gl: Rc::clone(gl) }
  }
  pub fn create_cube(&self) -> VaoPtr<ShapeFactoryVertex> {
    let mut v_data: Vec<ShapeFactoryVertex> = Vec::new();
    for ix in 0..=1 {
      let x = (ix as f32) - 0.5;
      for iy in 0..=1 {
        let y = (iy as f32) - 0.5;
        for iz in 0..=1 {
          let z = (iz as f32) - 0.5;
          let position = Vec3::new(x, y, z);
          for normal in [
            x.signum() * Vec3::X,
            y.signum() * Vec3::Y,
            z.signum() * Vec3::Z,
          ] {
            v_data.push(ShapeFactoryVertex { position, normal })
          }
        }
      }
    }
    // x: [0, 3, 6, 9] + 12
    // y: [1, 4, 16, 13] + 6
    // z: [2, 8, 14, 20] + 3
    let i_data: Vec<IndexBufferType> = vec![
      0, 3, 6, 6, 9, 3, 12, 15, 18, 18, 21, 15, //
      1, 4, 16, 16, 13, 4, 7, 10, 22, 22, 19, 10, //
      2, 8, 14, 14, 20, 8, 5, 11, 17, 17, 23, 11, //
    ];
    let i_buffer = IndexBuffer::new(&self.gl, i_data);
    let v_buffer = VertexBuffer::new(&self.gl, v_data);
    Rc::new(RefCell::new(Vao::new(&self.gl, v_buffer, i_buffer)))
  }
}
