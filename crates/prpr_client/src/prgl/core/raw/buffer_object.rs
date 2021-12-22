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

pub fn index_to_color_attachments_enum(x: usize) -> u32 {
  match x {
    0 => gl::COLOR_ATTACHMENT0,
    1 => gl::COLOR_ATTACHMENT1,
    2 => gl::COLOR_ATTACHMENT2,
    3 => gl::COLOR_ATTACHMENT3,
    4 => gl::COLOR_ATTACHMENT4,
    5 => gl::COLOR_ATTACHMENT5,
    6 => gl::COLOR_ATTACHMENT6,
    7 => gl::COLOR_ATTACHMENT7,
    8 => gl::COLOR_ATTACHMENT8,
    9 => gl::COLOR_ATTACHMENT9,
    10 => gl::COLOR_ATTACHMENT10,
    11 => gl::COLOR_ATTACHMENT11,
    12 => gl::COLOR_ATTACHMENT12,
    13 => gl::COLOR_ATTACHMENT13,
    14 => gl::COLOR_ATTACHMENT14,
    15 => gl::COLOR_ATTACHMENT15,
    _ => {
      log::error(format!("to big index_to_color_attachments_enum {}", x));
      gl::COLOR_ATTACHMENT0
    }
  }
}
