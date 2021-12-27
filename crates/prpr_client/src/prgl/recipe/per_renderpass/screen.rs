use super::*;

pub struct Surface {
  render_pass: RenderPass,
  viewport: Rect<i32>,
  instance: Arc<Instance>,
}
impl Surface {
  pub fn new(instance: &Arc<Instance>) -> Self {
    let mut render_pass = RenderPass::new(instance.ctx());
    render_pass.set_use_default_framebuffer(true);
    Self {
      render_pass,
      viewport: instance.full_viewport(),
      instance: instance.clone(),
    }
  }
  pub fn set_clear_color(&mut self, value: Option<Vec4>) {
    self.render_pass.set_clear_color(value);
  }
  pub fn update(&mut self) {
    self.viewport = self.instance.full_viewport();
    self.render_pass.set_viewport(Some(&self.viewport));
  }
  pub fn aspect_ratio(&self) -> f32 {
    self.viewport.width as f32 / self.viewport.height as f32
  }
  pub fn add(&mut self, bindable: &dyn RenderPassBindable) {
    bindable.bind_renderpass(&mut self.render_pass);
  }
  pub fn bind(&self) -> DescriptorContext {
    self.render_pass.bind()
  }
}

pub struct Screen {
  render_pass: RenderPass,
}
impl Screen {
  // pub fn new(ctx: &ArcGlContext) -> Self {}
}
