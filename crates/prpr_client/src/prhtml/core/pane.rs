use super::*;

pub struct Pane {
  height: f32,
  width_by_width: f32,
  min_width: Option<f32>,
  max_width: Option<f32>,
  holder: HtmlElementHolder,
  fit_point: PaneFitPoint,
  is_dirty: bool,
  owns: Vec<Box<dyn HtmlElementHolderTrait>>,
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
  pub fn new(fit_point: PaneFitPoint, width_by_width: f32, height: f32) -> Self {
    let root = prhtml::Instance::root();
    let holder = HtmlElementHolder::new(root, "div");
    let mut result = Self {
      height,
      width_by_width,
      min_width: None,
      max_width: None,
      holder,
      fit_point,
      is_dirty: true,
      owns: vec![],
    };
    result.setup();
    result
  }
  pub fn set_width_by_width(&mut self, v: f32) {
    self.width_by_width = v;
    self.is_dirty = true;
  }
  pub fn set_height(&mut self, v: f32) {
    self.height = v;
    self.is_dirty = true;
  }
  pub fn set_max_width(&mut self, v: Option<f32>) {
    self.max_width = v;
    self.is_dirty = true;
  }
  pub fn set_min_width(&mut self, v: Option<f32>) {
    self.min_width = v;
    self.is_dirty = true;
  }
  fn setup(&mut self) {
    self.holder.set_by_name_impl("overflow", "scroll");
    self.holder.set_by_name_impl("position", "absolute");
    self.adjust();
  }
  fn adjust(&mut self) {
    let width = system::WholeScreen::width() as f32;
    let height = system::WholeScreen::height() as f32;
    let expected_height = EXPECTED_BROWSER_HEIGHT;
    let scale = height / expected_height;
    self.set_scale(scale, Why::ByOriginal);
    let aspect = width as f32 / height as f32;
    let h = self.height;
    let mut w = self.width_by_width * aspect;
    if let Some(mw) = self.max_width {
      w = w.min(mw);
    }
    if let Some(mw) = self.min_width {
      w = w.max(mw);
    }
    self
      .holder
      .set_by_name_impl("width", &convert_percent_str(w));
    self
      .holder
      .set_by_name_impl("height", &convert_percent_str(h));
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
    self.holder.set_by_name_impl("top", &px(y));
    self.holder.set_by_name_impl("left", &px(x));
    self.is_dirty = false;
  }
}
impl NeedUpdate for Pane {
  fn update(&mut self) {
    if system::WholeScreen::is_size_changed() || self.is_dirty {
      self.adjust();
    }
  }
}

impl HtmlElementHolderTrait for Pane {
  fn holder(&self) -> &HtmlElementHolder {
    &self.holder
  }
}
impl ElementHolderContainerTrait for Pane {
  fn holder_container(&self) -> &web_sys::HtmlElement {
    &self.holder.raw_element()
  }
  fn own(&mut self, elem: Box<dyn HtmlElementHolderTrait>) {
    self.owns.push(elem);
  }
}
impl HtmlBackgroundTrait for Pane {}
impl HtmlBoxTrait for Pane {}
impl HtmlTextConfigurableTrait for Pane {}
impl HtmlContainerTrait for Pane {}
