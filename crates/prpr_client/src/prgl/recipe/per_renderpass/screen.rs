use super::*;

pub struct Surface {
  render_pass: RenderPass,
}

impl Surface {
  pub fn new() -> Self {
    let mut render_pass = RenderPass::new();
    render_pass.set_use_default_buffer(true);
    Self { render_pass }
  }
  pub fn set_clear_color(&mut self, value: Option<Vec4>) {
    self.render_pass.set_clear_color(value);
  }
  pub fn add(&mut self, bindable: &dyn RenderPassBindable) {
    bindable.bind_renderpass(&mut self.render_pass);
  }
  pub fn bind(&mut self, outer_ctx: &Arc<DescriptorContext>) -> Arc<DescriptorContext> {
    self.render_pass.set_viewport(Some(&Instance::viewport()));
    self.render_pass.bind(outer_ctx)
  }
}

pub struct Screen {
  render_pass: RenderPass,
  viewport: Rect<i32>,
}
impl Screen {
  // pub fn new(ctx: &ArcGlContext) -> Self {}
}
