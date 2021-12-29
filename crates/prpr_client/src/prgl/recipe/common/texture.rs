use super::*;

pub struct TextureRecipe {}

impl TextureRecipe {
  pub fn new_fullscreen(format: PixelFormat) -> ArcOwner<Texture> {
    let max_viewport = prgl::Instance::max_viewport();
    ArcOwner::new(Texture::new_uninitialized(&Texture2dDescriptor {
      width: max_viewport.width as usize,
      height: max_viewport.height as usize,
      format,
      mipmap: true,
    }))
  }
  pub fn new_fullscreen_depth() -> ArcOwner<Texture> {
    let max_viewport = prgl::Instance::max_viewport();
    ArcOwner::new(Texture::new_uninitialized(&Texture2dDescriptor {
      width: max_viewport.width as usize,
      height: max_viewport.height as usize,
      format: PixelFormat::Depth24,
      mipmap: false,
    }))
  }
}
