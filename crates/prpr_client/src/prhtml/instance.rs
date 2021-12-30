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
  position: math::Vec2, // 中心の位置(正規化座標)
  size: math::Vec2,     // width,height(正規化座標)
  is_dirty: bool,
  raw_element: web_sys::HtmlDivElement,
}
impl FloatingBox {
  pub fn new(root: &web_sys::HtmlDivElement) -> Self {
    let raw_element = js::html::append_div(root);
    let mut result = Self {
      position: math::Vec2::ZERO,
      size: math::Vec2::ZERO,
      is_dirty: true,
      raw_element,
    };
    result.adjust().ok();
    result
  }
  pub fn set_position(&mut self, position: Vec2) {
    self.position = position;
    self.is_dirty = true;
  }
  pub fn set_size(&mut self, size: Vec2) {
    self.size = size;
    self.is_dirty = true;
  }
  pub fn set_text_debug(&mut self, text: &str) {
    self.raw_element.set_inner_text(&text);
  }
  pub fn adjust(&mut self) -> Result<(), wasm_bindgen::JsValue> {
    if !self.is_dirty {
      if !system::WholeScreen::is_size_changed() {
        return Result::Ok(());
      }
    }
    let width = system::WholeScreen::width() as f32;
    let height = system::WholeScreen::height() as f32;
    let style = self.raw_element.style();
    // 小さすぎると崩れるので、仮である程度の大きさのheightを仮定している
    let expected_height = 1000.0;
    let percent = |f: f32| format!("{}px", f * expected_height * 0.01);
    // color: linear-gradient / radial-gradient
    style.set_property("border-style", "solid")?;
    style.set_property("border-color", "black")?;
    style.set_property("border-radius", &percent(1.4))?;
    style.set_property("border-width", &percent(0.4))?;
    style.set_property("border-style", "solid")?; // solid, double, hidden, dashed
    style.set_property("padding", &percent(1.0))?;
    style.set_property("background-color", "rgba(255,255,255,0.5)")?;
    style.set_property("cursor", "pointer")?; // move, wait, ...etc
    style.set_property("filter", "blur(1px)")?;
    // brightness, contrast, grayscale(), invert(), opacity
    // saturate, sepia,
    //
    // style.set_property("text-align", "center")?;
    style.set_property("font-size", &percent(2.4))?;
    style.set_property("line-height", &percent(3.0))?;
    style.set_property("letter-spacing", &percent(0.1))?;
    style.set_property("color", "black")?;
    // style.set_property("font-style", "italic")?;
    // style.set_property("font-weight", "bolder")?;
    // style.set_property("text-decoration", "underline")?; // line-through
    style.set_property("text-shadow", "3px 3px 2px rgba(125,125,125,0.8)")?;
    //
    // transform: trainlate, rotate
    style.set_property("overflow", "scroll")?;
    style.set_property("position", "absolute")?;
    let scale = height / expected_height;
    style.set_property("transform", &format!("scale({})", scale))?;
    style.set_property("transform-origin", "center")?;
    style.set_property("width", &percent(self.size.x * 100.0))?;
    style.set_property("height", &percent(self.size.y * 100.0))?;
    let y = -self.position.y * height + 0.5 * height - self.size.y * 0.5 * expected_height;
    let x = self.position.x * height + 0.5 * width - self.size.x * 0.5 * expected_height;
    let px = |f: f32| format!("{}px", f);
    style.set_property("top", &px(y))?;
    style.set_property("left", &px(x))?;
    // style.set_property("z-index", &z_index.to_string());
    // style.set_property("display", "none");
    self.is_dirty = false;
    Result::Ok(())
  }
}
impl Updatable for FloatingBox {
  fn update(&mut self) {
    self.adjust().ok();
  }
}
impl Drop for FloatingBox {
  fn drop(&mut self) {
    self.raw_element.remove();
  }
}
