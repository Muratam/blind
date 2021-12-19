use super::*;

pub struct Texture {
  gl: ArcGlContext,
  // raw_texture: RawTexture,
}

impl Texture {
  // sampler / textureimage2d
  pub fn new(gl: &ArcGlContext) -> Self {
    // let raw_texture = RawTexture::new(gl);
    Self {
      gl: gl.clone(),
      // raw_texture,
    }
  }
}
