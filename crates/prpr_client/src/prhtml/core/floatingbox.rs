use super::*;

pub struct FloatingBox {
  position: Vec2, // 中心の位置(正規化座標)
  size: Vec2,     // width,height(正規化座標)
  is_dirty: bool,
  raw_element: web_sys::HtmlDivElement,
}
impl FloatingBox {
  pub fn new() -> Self {
    let root = prhtml::Instance::root();
    let raw_element = js::html::append_div(root);
    let mut result = Self {
      position: Vec2::ZERO,
      size: Vec2::ZERO,
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
    // style.set_property("text-decoration", "underline")?; // line-through
    // style.set_property("text-align", "center")?;
    let width = system::WholeScreen::width() as f32;
    let height = system::WholeScreen::height() as f32;
    let style = self.raw_element.style();
    let expected_height = 1000.0;
    style.set_property("cursor", "pointer")?; // move, wait, ...etc
    style.set_property("overflow", "scroll")?;
    style.set_property("position", "absolute")?;
    let scale = height / expected_height;
    // transform: trainlate, rotate
    style.set_property("transform", &format!("scale({})", scale))?;
    style.set_property("transform-origin", "center")?;
    let percent = |f: f32| format!("{}px", f * expected_height * 0.01);
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
impl ContainerTrait for FloatingBox {
  fn get_raw_element(&self) -> &web_sys::HtmlElement {
    &wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlElement>(&self.raw_element)
      .expect("failed to cast to CanvasRenderingContext2d")
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
