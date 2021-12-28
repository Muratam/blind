use super::layers::Layers;
use super::*;
use crate::prgl;

pub struct Core {
  layers: Layers,
  // audio
  // input
  // etc...
  frame: i64,
  pre_now_milli_sec: f64,
  processed_milli_sec: f64,
}

impl Core {
  pub fn new() -> Self {
    let layers = Layers::new();
    prgl::Instance::set(layers.main_3d_context());
    Self {
      layers,
      pre_now_milli_sec: js::date::now_millisec(),
      processed_milli_sec: 0.0,
      frame: 0,
    }
  }
  pub fn pre_update(&mut self) {
    self.frame += 1;
    self.layers.adjust_screen_size();
    prgl::Instance::update_size(self.layers.width(), self.layers.height());
    self.pre_now_milli_sec = js::date::now_millisec();
  }
  pub fn post_update(&mut self) {
    self.processed_milli_sec = js::date::now_millisec() - self.pre_now_milli_sec;
    prgl::Instance::flush();
  }
  pub fn frame(&self) -> i64 {
    self.frame
  }
  pub fn processed_time(&self) -> f64 {
    self.processed_milli_sec
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
