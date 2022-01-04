use super::*;

static INSTANCE: OnceCell<Instance> = OnceCell::new();
unsafe impl Send for Instance {}
unsafe impl Sync for Instance {}

pub struct Instance {
  root: SRc<web_sys::HtmlDivElement>,
}
impl Instance {
  pub fn root() -> &'static web_sys::HtmlDivElement {
    &INSTANCE
      .get()
      .expect("prhtml::Instance is not initialized")
      .root
  }
  pub fn set(root: &SRc<web_sys::HtmlDivElement>) {
    INSTANCE.set(Self { root: root.clone() }).ok();
  }
}
