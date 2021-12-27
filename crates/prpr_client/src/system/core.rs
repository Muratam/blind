use super::layers::Layers;
use super::*;
use crate::prgl;
use std::sync::*;

pub struct Core {
  layers: Layers,
  // audio
  // input
  // etc...
  frame: i64,
  main_prgl: Arc<prgl::Instance>,
  pre_now_milli_sec: f64,
  processed_milli_sec: f64,
}

impl Core {
  pub fn new() -> Self {
    let layers = Layers::new();
    let main_prgl = prgl::Instance::new(layers.main_3d_context());
    Self {
      layers,
      main_prgl: Arc::new(main_prgl),
      pre_now_milli_sec: js::date::now_millisec(),
      processed_milli_sec: 0.0,
      frame: 0,
    }
  }
  pub fn pre_update(&mut self) {
    self.frame += 1;
    self.layers.adjust_screen_size();
    self
      .main_prgl
      .update_size(self.layers.width(), self.layers.height());
    self.pre_now_milli_sec = js::date::now_millisec();
  }
  pub fn post_update(&mut self) {
    self.processed_milli_sec = js::date::now_millisec() - self.pre_now_milli_sec;
    self.main_prgl.flush();
  }
  pub fn frame(&self) -> i64 {
    self.frame
  }
  pub fn processed_time(&self) -> f64 {
    self.processed_milli_sec
  }
  pub fn main_prgl(&self) -> &Arc<prgl::Instance> {
    &self.main_prgl
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
