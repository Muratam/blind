use crate::js;
use crate::prgl;
mod layers;
use layers::Layers;

pub trait Runnable {
  fn update(&mut self);
}

pub fn run<T: 'static + Runnable>(mut runnable: T) {
  js::start_animation_frame_loop(Box::new(move || {
    runnable.update();
  }))
}

pub struct System {
  layers: Layers,
  // audio
  // input
  // etc...
  frame: i64,
  main_prgl: prgl::Instance,
}
impl System {
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
  pub fn frame(&self) -> i64 {
    self.frame
  }
  pub fn main_prgl(&self) -> &prgl::Instance {
    &self.main_prgl
  }
  // depracated(もっとラップする)
  pub fn main_2d_context(&self) -> web_sys::CanvasRenderingContext2d {
    self.layers.get_main_2d_context()
  }
  // depracated(もっとラップする)
  pub fn html_layer(&self) -> &web_sys::HtmlDivElement {
    self.layers.get_html_layer()
  }
}
