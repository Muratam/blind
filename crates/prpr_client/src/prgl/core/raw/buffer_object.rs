use super::*;
pub struct RawRenderBuffer {
  gl: Arc<GlContext>,
  raw_renderbuffer: web_sys::WebGlRenderbuffer,
}
impl RawRenderBuffer {
  pub fn new(gl: &Arc<GlContext>) -> Self {
    let raw_renderbuffer = gl
      .create_renderbuffer()
      .expect("failed to create render buffer");
    Self {
      gl: Arc::clone(gl),
      raw_renderbuffer,
    }
  }
  pub fn raw_renderbuffer(&self) -> &web_sys::WebGlRenderbuffer {
    &self.raw_renderbuffer
  }
}
impl Drop for RawRenderBuffer {
  fn drop(&mut self) {
    self.gl.delete_renderbuffer(Some(&self.raw_renderbuffer));
  }
}

pub struct RawFrameBuffer {
  gl: Arc<GlContext>,
  raw_framebuffer: web_sys::WebGlFramebuffer,
}
impl RawFrameBuffer {
  pub fn new(gl: &Arc<GlContext>) -> Self {
    let raw_framebuffer = gl
      .create_framebuffer()
      .expect("failed to create frame buffer");
    Self {
      gl: Arc::clone(gl),
      raw_framebuffer,
    }
  }
  pub fn raw_framebuffer(&self) -> &web_sys::WebGlFramebuffer {
    &self.raw_framebuffer
  }
}
impl Drop for RawFrameBuffer {
  fn drop(&mut self) {
    self.gl.delete_framebuffer(Some(&self.raw_framebuffer));
  }
}
