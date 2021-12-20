use super::*;
pub struct RawRenderBuffer {
  ctx: ArcGlContext,
  raw_renderbuffer: web_sys::WebGlRenderbuffer,
}
impl RawRenderBuffer {
  pub fn new(ctx: &ArcGlContext) -> Self {
    let raw_renderbuffer = ctx
      .create_renderbuffer()
      .expect("failed to create render buffer");
    Self {
      ctx: ctx.clone(),
      raw_renderbuffer,
    }
  }
  pub fn raw_renderbuffer(&self) -> &web_sys::WebGlRenderbuffer {
    &self.raw_renderbuffer
  }
}
impl Drop for RawRenderBuffer {
  fn drop(&mut self) {
    self.ctx.delete_renderbuffer(Some(&self.raw_renderbuffer));
  }
}

pub struct RawFrameBuffer {
  ctx: ArcGlContext,
  raw_framebuffer: web_sys::WebGlFramebuffer,
}
impl RawFrameBuffer {
  pub fn new(ctx: &ArcGlContext) -> Self {
    let raw_framebuffer = ctx
      .create_framebuffer()
      .expect("failed to create frame buffer");
    Self {
      ctx: ctx.clone(),
      raw_framebuffer,
    }
  }
  pub fn raw_framebuffer(&self) -> &web_sys::WebGlFramebuffer {
    &self.raw_framebuffer
  }
}
impl Drop for RawFrameBuffer {
  fn drop(&mut self) {
    self.ctx.delete_framebuffer(Some(&self.raw_framebuffer));
  }
}
