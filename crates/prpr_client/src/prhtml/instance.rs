use super::*;

// WARN: 多分別スレッドから実行できない
use once_cell::sync::OnceCell;
static INSTANCE: OnceCell<Instance> = OnceCell::new();
unsafe impl Send for Instance {}
unsafe impl Sync for Instance {}

pub struct Instance {
  root: Arc<web_sys::HtmlDivElement>,
}
impl Instance {
  pub fn get() -> &'static Self {
    INSTANCE.get().expect("prhtml::Instance is not initialized")
  }
  pub fn root() -> &'static web_sys::HtmlDivElement {
    &Self::get().root
  }
  pub fn set(root: &Arc<web_sys::HtmlDivElement>) {
    // 一度生成したら固定
    let instance = Self { root: root.clone() };
    INSTANCE.set(instance).ok();
  }
}
pub struct HtmlFloatingBox {
  pos: math::Vec2,  // 中心の位置(正規化座標)
  size: math::Vec2, // width,height(正規化座標)
  raw_element: web_sys::HtmlDivElement,
}
impl HtmlFloatingBox {
  pub fn new(root: &web_sys::HtmlDivElement) -> Self {
    let raw_element = js::html::append_div(root);
    Self {
      pos: math::Vec2::ZERO,
      size: math::Vec2::ONE * 0.25,
      raw_element: raw_element,
    }
  }
}
