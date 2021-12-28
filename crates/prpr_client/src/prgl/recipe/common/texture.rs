use super::*;

pub struct TextureRecipe {}

impl TextureRecipe {
  pub fn new_fullscreen(format: PixelFormat) -> Arc<Texture> {
    let max_viewport = prgl::Instance::max_viewport();
    Arc::new(Texture::new_uninitialized(&Texture2dDescriptor {
      width: max_viewport.width as usize,
      height: max_viewport.height as usize,
      format,
      mipmap: true,
    }))
  }
  pub fn new_fullscreen_depth() -> Arc<Texture> {
    let max_viewport = prgl::Instance::max_viewport();
    Arc::new(Texture::new_uninitialized(&Texture2dDescriptor {
      width: max_viewport.width as usize,
      height: max_viewport.height as usize,
      format: PixelFormat::Depth24,
      mipmap: false,
    }))
  }
}
