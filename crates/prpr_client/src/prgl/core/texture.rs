use super::*;
pub struct Texture {
  gl: Rc<GlContext>,
}

impl Texture {
  pub fn new(gl: &Rc<GlContext>) -> Self {
    Self { gl: Rc::clone(gl) }
  }
}
