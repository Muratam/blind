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
    // 画面の縦の長さが1000pxとして指定する
    // あまりにも小さいpxを指定するとフォントが対応していないことがあり崩れるため
    let px = |f: f32| format!("{}px", f);
    style.set_property("border-style", "solid")?;
    style.set_property("border-color", "red")?;
    style.set_property("border-width", &px(4.0))?;
    style.set_property("font-size", &px(24.0))?;
    style.set_property("padding", &px(10.0))?;
    style.set_property("background-color", "rgba(255,255,255,0.5)")?;
    style.set_property("position", "absolute")?;
    let expected_height = 1000.0;
    let scale = height / expected_height;
    style.set_property("transform", &format!("scale({})", scale))?;
    style.set_property("transform-origin", "center")?;
    style.set_property("width", &px(self.size.x * expected_height))?;
    style.set_property("height", &px(self.size.y * expected_height))?;
    let y = self.pos.y * height + 0.5 * height - self.size.y * 0.5 * expected_height;
    let x = self.pos.x * height + 0.5 * width - self.size.x * 0.5 * expected_height;
    style.set_property("top", &px(y))?;
    style.set_property("left", &px(x))
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
