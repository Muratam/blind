use super::time::TimeGlobal;
use super::*;
use crate::prgl;
pub struct Core {
  layers: Layers,
  // audio
  // input
  // etc...
}

impl Core {
  pub fn new() -> Self {
    let layers = Layers::new();
    prgl::Instance::set(layers.main_3d_context());
    TimeGlobal::initialize();
    Self { layers }
  }
  pub fn pre_update(&mut self) {
    self.layers.adjust_screen_size();
    prgl::Instance::update_size(self.layers.width(), self.layers.height());
    TimeGlobal::write().pre_update();
  }
  pub fn post_update(&mut self) {
    TimeGlobal::write().post_update();
    prgl::Instance::flush();
  }
  // depracated(もっとラップする)
  pub fn main_2d_context(&self) -> web_sys::CanvasRenderingContext2d {
    self.layers.main_2d_context()
  }
  // depracated(もっとラップする)
  pub fn html_layer(&self) -> &web_sys::HtmlDivElement {
    self.layers.html_layer()
  }
}
