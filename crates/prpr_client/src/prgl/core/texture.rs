use super::*;

pub struct Texture {
  gl: ArcGlContext,
  // raw_texture: RawTexture,
}

impl Texture {
  // sampler / textureimage2d
  pub fn new(gl: &ArcGlContext) -> Self {
    // once
    // let raw_texture = RawTexture::new(gl);
    Self {
      gl: gl.clone(),
      // raw_texture,
    }
  }
}

// ShaderTemplateで生成したmappingを引数に取ってバインドに使う
pub struct TextureMapping {}
// gl.active_texture(to_slot(0));
// gl.bind_texture(gl::TEXTURE_2D, texture);
// gl.uniform1i(gl.getUniformLocation(program, "uSampler"), 0);
