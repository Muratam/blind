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
    result.adjust();
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
  fn adjust(&mut self) {
    if !self.is_dirty {
      if !system::WholeScreen::is_size_changed() {
        return;
      }
    }
    // css animation?
    // style.set_property("text-decoration", "underline")?; // line-through
    // style.set_property("text-align", "center")?;
    let width = system::WholeScreen::width() as f32;
    let height = system::WholeScreen::height() as f32;
    let expected_height = 1000.0;
    self.set_by_name_impl("cursor", "pointer"); // move, wait, ...etc
    self.set_by_name_impl("overflow", "scroll");
    self.set_by_name_impl("position", "absolute");
    let scale = height / expected_height;
    // transform: trainlate, rotate
    self.set_by_name_impl("transform", &format!("scale({})", scale));
    self.set_by_name_impl("transform-origin", "center");
    let percent = |f: f32| format!("{}px", f * expected_height * 0.01);
    self.set_by_name_impl("width", &percent(self.size.x * 100.0));
    self.set_by_name_impl("height", &percent(self.size.y * 100.0));
    let y = -self.position.y * height + 0.5 * height - self.size.y * 0.5 * expected_height;
    let x = self.position.x * height + 0.5 * width - self.size.x * 0.5 * expected_height;
    let px = |f: f32| format!("{}px", f);
    self.set_by_name_impl("top", &px(y));
    self.set_by_name_impl("left", &px(x));
    // style.set_property("z-index", &z_index.to_string());
    // style.set_property("display", "none");
    self.is_dirty = false;
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
    self.adjust();
  }
}
impl Drop for FloatingBox {
  fn drop(&mut self) {
    self.raw_element.remove();
  }
}
