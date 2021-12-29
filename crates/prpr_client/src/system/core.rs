use super::*;
use crate::prgl;
pub struct Core {
  layers: Layers,
  // audio
  // etc...
}

struct HtmlFloatingBox {
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

impl Core {
  pub fn new() -> Self {
    let layers = Layers::new();
    prgl::Instance::set(layers.main_3d_context());
    time::TimeImpl::initialize_global();
    prgl::RenderPassExecuterImpl::initialize_global();
    UpdaterImpl::initialize_global();
    EventHolderImpl::initialize_global(layers.html_layer());
    Self { layers }
  }
  pub fn pre_update(&mut self) {
    self.layers.adjust_screen_size();
    prgl::Instance::update_size(self.layers.width(), self.layers.height());
    time::TimeImpl::write_global().pre_update();
    EventHolderImpl::write_global().update();
  }
  pub fn update(&mut self) {
    UpdaterImpl::write_global().execute();
    self.debug_update();
  }
  fn debug_update(&mut self) {
    // TODO: 消す
    if true {
      // HtmlBox
      let html_layer = self.html_layer();
      let mut text = format!("{} ms\n", Time::processed_milli_sec());
      text += &format!("({}, {})\n", input::Mouse::x(), input::Mouse::y());
      html_layer.set_text_content(Some(&text));
    }
  }
  pub fn post_update(&mut self) {
    prgl::RenderPassExecuterImpl::write_global().execute();
    prgl::Instance::flush();
    time::TimeImpl::write_global().post_update();
  }
  // depracated(もっとラップする)
  pub fn html_layer(&self) -> &web_sys::HtmlDivElement {
    self.layers.html_layer()
  }
}
