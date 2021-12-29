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
    time::TimeImpl::initialize();
    prgl::RenderPassExecuterImpl::initialize_global();
    UpdaterImpl::initialize_global();
    Self { layers }
  }
  pub fn pre_update(&mut self) {
    self.layers.adjust_screen_size();
    prgl::Instance::update_size(self.layers.width(), self.layers.height());
    time::TimeImpl::write_global().pre_update();
  }
  pub fn update(&mut self) {
    // TODO: 消す
    if true {
      let ctx = self.main_2d_context();
      use std::f64::consts::PI;
      ctx.begin_path();
      ctx.arc(75.0, 75.0, 50.0, 0.0, PI * 2.0).ok();
      ctx.move_to(110.0, 75.0);
      ctx.stroke();
      let html_layer = self.html_layer();
      let text = format!("{} ms", Time::processed_milli_sec());
      html_layer.set_text_content(Some(&text));
    }
    UpdaterImpl::write_global().execute();
  }
  pub fn post_update(&mut self) {
    prgl::RenderPassExecuterImpl::write_global().execute();
    time::TimeImpl::write_global().post_update();
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
