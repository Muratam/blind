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
    prgl::RenderPassExecuter::global_initialize();
    UpdaterExecuter::global_initialize();
    Self { layers }
  }
  pub fn pre_update(&mut self) {
    self.layers.adjust_screen_size();
    prgl::Instance::update_size(self.layers.width(), self.layers.height());
    TimeGlobal::write_lock().pre_update();
  }
  pub fn post_update(&mut self) {
    UpdaterExecuter::global_write_lock().execute();
    prgl::RenderPassExecuter::global_write_lock().execute();
    TimeGlobal::write_lock().post_update();
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
