use super::*;

pub struct Surface {
  render_pass: RenderPass,
}
impl Surface {
  pub fn new(ctx: &ArcGlContext) -> Self {
    let mut render_pass = RenderPass::new(ctx);
    render_pass.set_use_default_framebuffer(true);
    Self { render_pass }
  }
  pub fn set_clear_color(&mut self, value: Option<Vec4>) {
    self.render_pass.set_clear_color(value);
  }
  pub fn update(&mut self, instance: &Instance) {
    self
      .render_pass
      .set_viewport(Some(&instance.full_viewport()));
  }
  pub fn bind(&self) {
    self.render_pass.bind();
  }
}

pub struct Screen {
  render_pass: RenderPass,
}
impl Screen {
  // pub fn new(ctx: &ArcGlContext) -> Self {}
}
