use super::layers::Layers;
use crate::prgl;

pub struct Core {
  layers: Layers,
  // audio
  // input
  // etc...
  frame: i64,
  main_prgl: prgl::Instance,
}

impl Core {
  pub fn new() -> Self {
    let layers = Layers::new();
    let main_prgl = prgl::Instance::new(layers.get_main_3d_context());
    Self {
      layers,
      main_prgl,
      frame: 0,
    }
  }
  pub fn update(&mut self) {
    self.frame += 1;
    self.layers.adjust_screen_size();
  }
  pub fn get_frame(&self) -> i64 {
    self.frame
  }
  pub fn get_main_prgl(&self) -> &prgl::Instance {
    &self.main_prgl
  }
  // depracated(もっとラップする)
  pub fn get_main_2d_context(&self) -> web_sys::CanvasRenderingContext2d {
    self.layers.get_main_2d_context()
  }
  // depracated(もっとラップする)
  pub fn get_html_layer(&self) -> &web_sys::HtmlDivElement {
    self.layers.get_html_layer()
  }
}
