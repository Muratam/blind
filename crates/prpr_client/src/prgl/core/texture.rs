use super::*;

pub struct Texture {
  ctx: ArcGlContext,
  // raw_texture: RawTexture,
}

impl Texture {
  // sampler / textureimage2d
  pub fn new(ctx: &ArcGlContext) -> Self {
    // once
    // let raw_texture = RawTexture::new(ctx);
    Self {
      ctx: ctx.clone(),
      // raw_texture,
    }
  }
}

// ShaderTemplateで生成したmappingを引数に取ってバインドに使う
pub struct TextureMapping {}
// ctx.active_texture(to_slot(0));
// ctx.bind_texture(gl::TEXTURE_2D, texture);
// ctx.uniform1i(ctx.getUniformLocation(program, "uSampler"), 0);
