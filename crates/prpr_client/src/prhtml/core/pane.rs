use super::*;

pub struct Pane {
  scale: f32,
  rotate_deg: f32,
  offset: Vec2,
  height: f32,
  width_by_width: f32,
  min_width: Option<f32>,
  max_width: Option<f32>,
  raw_element: web_sys::HtmlDivElement,
  fit_point: PaneFitPoint,
  is_dirty: bool,
}
#[derive(Clone, Copy)]
pub enum PaneFitPoint {
  LeftTop,
  Left,
  LeftBottom,
  Top,
  Center,
  Bottom,
  RightTop,
  Right,
  RightBottom,
}
impl Pane {
  // FloatingPaneはどこでも置ける＋ドラッグできる。ただし見切れることがある。嫌いなのでいらない。
  // {Left, Center, Right} * {Top, Center, Bottom} のどこに吸着するかを指定する
  // width, height は画面の何％かで指定する。x,y はoffsetを指定する。
  // height は固定。widthは伸びる可能性がある。
  // - 最大%(これ以上伸びない)(heightに対して)を指定可能
  // - 最小%(これ以上縮まない)(heightに対して)も指定可能(ただし衝突する可能性がありそう)
  // 回転・スケーリング・移動は差分を指定して可能。アニメーション向け
  // - offsetはtransformで
  // - サイズは固定。
  pub fn new(fit_point: PaneFitPoint, width_by_width: f32, height: f32) -> Self {
    let root = prhtml::Instance::root();
    let raw_element = js::html::append_div(root);
    let mut result = Self {
      scale: 1.0,
      rotate_deg: 0.0,
      offset: Vec2::ZERO,
      height,
      width_by_width,
      min_width: None,
      max_width: None,
      raw_element,
      fit_point,
      is_dirty: true,
    };
    result.setup();
    result
  }
  pub fn set_max_width(&mut self, v: Option<f32>) {
    self.max_width = v;
    self.is_dirty = true;
  }
  pub fn set_min_width(&mut self, v: Option<f32>) {
    self.min_width = v;
    self.is_dirty = true;
  }
  pub fn set_scale(&mut self, v: f32) {
    self.scale = v;
    self.is_dirty = true;
  }
  pub fn set_rotate_deg(&mut self, v: f32) {
    self.rotate_deg = v;
    self.is_dirty = true;
  }
  pub fn set_offset(&mut self, v: Vec2) {
    self.offset = v;
    self.is_dirty = true;
  }
  pub fn set_text_debug(&mut self, text: &str) {
    self.raw_element.set_inner_text(&text);
  }
  fn setup(&mut self) {
    self.set_by_name_impl("overflow", "scroll");
    self.set_by_name_impl("position", "absolute");
    // そのうち使うかも？
    // self.set_by_name_impl("display", "flex");
    // self.set_by_name_impl("transform-origin", "center");
    self.adjust();
  }
  fn adjust(&mut self) {
    let width = system::WholeScreen::width() as f32;
    let height = system::WholeScreen::height() as f32;
    let expected_height = EXPECTED_BROWSER_HEIGHT;
    let scale = height / expected_height * self.scale;
    self.set_by_name_impl(
      "transform",
      &format!(
        "translate({},{}) scale({}) rotate({}deg)",
        convert_percent_str(self.offset.x),
        convert_percent_str(self.offset.y),
        scale,
        self.rotate_deg
      ),
    );
    let aspect = width as f32 / height as f32;
    let h = self.height;
    let mut w = self.width_by_width * aspect;
    if let Some(mw) = self.max_width {
      w = w.min(mw);
    }
    if let Some(mw) = self.min_width {
      w = w.max(mw);
    }
    self.set_by_name_impl("width", &convert_percent_str(w));
    self.set_by_name_impl("height", &convert_percent_str(h));
    let position = match self.fit_point {
      PaneFitPoint::LeftTop => Vec2::new(-0.5, 0.5),
      PaneFitPoint::Left => Vec2::new(-0.5, 0.0),
      PaneFitPoint::LeftBottom => Vec2::new(-0.5, -0.5),
      PaneFitPoint::Top => Vec2::new(0.0, 0.5),
      PaneFitPoint::Center => Vec2::new(0.0, 0.0),
      PaneFitPoint::Bottom => Vec2::new(0.0, -0.5),
      PaneFitPoint::RightTop => Vec2::new(0.5, 0.5),
      PaneFitPoint::Right => Vec2::new(0.5, 0.0),
      PaneFitPoint::RightBottom => Vec2::new(0.5, -0.5),
    };
    let w = w * 0.01;
    let h = h * 0.01;
    let y = (-position.y + 0.5) * height - h * 0.5 * expected_height + h * position.y * height;
    let x = (position.x + 0.5) * width - w * 0.5 * expected_height - w * position.x * height;
    let px = |f: f32| format!("{}px", f);
    self.set_by_name_impl("top", &px(y));
    self.set_by_name_impl("left", &px(x));
    self.is_dirty = false;
  }
}
impl Updatable for Pane {
  fn update(&mut self) {
    if system::WholeScreen::is_size_changed() || self.is_dirty {
      self.adjust();
    }
  }
}

impl HtmlElementHolder for Pane {
  fn get_raw_element(&self) -> &web_sys::HtmlElement {
    &wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlElement>(&self.raw_element)
      .expect("failed to cast to CanvasRenderingContext2d")
  }
}
impl HtmlTextHolderTrait for Pane {}
impl HtmlContainerTrait for Pane {}

impl Drop for Pane {
  fn drop(&mut self) {
    self.raw_element.remove();
  }
}
