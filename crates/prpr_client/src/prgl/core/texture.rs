use super::*;
pub struct Texture {
  gl: Rc<GlContext>,
  // raw_texture: RawTexture,
}

impl Texture {
  // sampler / textureimage2d
  pub fn new(gl: &Rc<GlContext>) -> Self {
    // let raw_texture = RawTexture::new(gl);
    Self {
      gl: Rc::clone(gl),
      // raw_texture,
    }
  }
}
