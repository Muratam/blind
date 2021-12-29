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
  pub fn root() -> &'static web_sys::HtmlDivElement {
    &INSTANCE
      .get()
      .expect("prhtml::Instance is not initialized")
      .root
  }
  pub fn set(root: &Arc<web_sys::HtmlDivElement>) {
    INSTANCE.set(Self { root: root.clone() }).ok();
  }
}
pub struct FloatingBox {
  pub pos: math::Vec2,  // 中心の位置(正規化座標)
  pub size: math::Vec2, // width,height(正規化座標)
  pub raw_element: web_sys::HtmlDivElement,
}
impl FloatingBox {
  pub fn new(root: &web_sys::HtmlDivElement) -> Self {
    let raw_element = js::html::append_div(root);
    let result = Self {
      pos: math::Vec2::ZERO,
      size: math::Vec2::ONE * 0.25,
      raw_element,
    };
    result.adjust().ok();
    result
  }
  fn adjust(&self) -> Result<(), wasm_bindgen::JsValue> {
    if !system::WholeScreen::is_size_changed() {
      return Result::Ok(());
    }
    let width = system::WholeScreen::width() as f32;
    let height = system::WholeScreen::height() as f32;
    let style = self.raw_element.style();
    style.set_property("border-style", "solid")?;
    style.set_property("border-color", "red")?;
    style.set_property("border-width", "4px")?;
    style.set_property("background-color", "rgba(255,255,255,0.5)")?;
    style.set_property("position", "absolute")?;
    style.set_property("width", &format!("{}px", self.size.x * height))?;
    style.set_property("height", &format!("{}px", self.size.y * height))?;
    style.set_property(
      "top",
      &format!("{}px", (self.pos.y + 0.5 - self.size.y * 0.5) * height),
    )?;
    style.set_property(
      "left",
      &format!(
        "{}px",
        (self.pos.x - self.size.x * 0.5) * height + 0.5 * width
      ),
    )
    // style.set_property("z-index", &z_index.to_string()).ok();
  }
}
impl Updatable for FloatingBox {
  fn update(&mut self) {
    let mut text = format!("{} ms\n", Time::processed_milli_sec());
    text += &format!("({}, {})\n", input::Mouse::x(), input::Mouse::y());
    self.raw_element.set_text_content(Some(&text));
    self.adjust().ok();
  }
}
impl Drop for FloatingBox {
  fn drop(&mut self) {
    self.raw_element.remove();
  }
}
