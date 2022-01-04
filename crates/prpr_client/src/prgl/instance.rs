use super::*;

static INSTANCE: OnceCell<Instance> = OnceCell::new();
unsafe impl Send for Instance {}
unsafe impl Sync for Instance {}

pub struct Instance {
  ctx: web_sys::WebGl2RenderingContext,
}
impl Instance {
  pub fn ctx() -> &'static web_sys::WebGl2RenderingContext {
    &INSTANCE
      .get()
      .expect("prgl::Instance is not initialized")
      .ctx
  }
  pub fn set(ctx: web_sys::WebGl2RenderingContext) {
    INSTANCE.set(Self { ctx }).ok();
  }
  pub fn flush() {
    Self::ctx().flush();
  }
}
