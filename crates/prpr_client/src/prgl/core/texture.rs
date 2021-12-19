use super::*;

pub struct Texture {
  gl: Arc<GlContext>,
  // raw_texture: RawTexture,
}

impl Texture {
  // sampler / textureimage2d
  pub fn new(gl: &Arc<GlContext>) -> Self {
    // let raw_texture = RawTexture::new(gl);
    Self {
      gl: Arc::clone(gl),
      // raw_texture,
    }
  }
}
