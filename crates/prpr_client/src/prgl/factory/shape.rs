use super::*;

pub struct ShapeFactory {
  gl: Rc<GlContext>,
}
crate::shader_attr! {
  struct ShapeFactoryVertex {
    position: vec3,
    normal: vec3,
    tangent: vec3,
    uv: vec2
  }
}
impl ShapeFactory {
  pub fn new(gl: &Rc<GlContext>) -> Self {
    Self { gl: Rc::clone(gl) }
  }
  pub fn create_cube(&self) -> VaoPtr<ShapeFactoryVertex> {
    let mut i_data: Vec<IndexBufferType> = Vec::new();
    let mut v_data: Vec<ShapeFactoryVertex> = Vec::new();
    for ix in 0..=1 {
      let x = (ix as f32) - 0.5;
      for iy in 0..=1 {
        let y = (iy as f32) - 0.5;
        for iz in 0..=1 {
          let z = (iz as f32) - 0.5;
        }
      }
    }
    let i_buffer = IndexBuffer::new(&self.gl, i_data);
    let v_buffer = VertexBuffer::new(&self.gl, v_data);
    Rc::new(RefCell::new(Vao::new(&self.gl, v_buffer, i_buffer)))
  }
}
