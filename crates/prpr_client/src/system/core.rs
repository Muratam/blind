use super::*;
use crate::prgl;
pub struct Core {
  layers: Layers,
  // audio
  // etc...
}

impl Core {
  pub fn new() -> Self {
    let layers = Layers::new();
    WholeScreen::initialize();
    prgl::Instance::set(layers.main_3d_context());
    prhtml::Instance::set(layers.html_layer());
    time::TimeImpl::initialize_global();
    prgl::RenderPassExecuterImpl::initialize_global();
    UpdaterImpl::initialize_global();
    EventHolderImpl::initialize_global(layers.html_layer());
    Self { layers }
  }
  pub fn pre_update(&mut self) {
    self.layers.adjust_screen_size();
    WholeScreen::update_size(self.layers.width(), self.layers.height());
    time::TimeImpl::write_global().pre_update();
    EventHolderImpl::write_global().update();
  }
  pub fn update(&mut self) {
    UpdaterImpl::write_global().execute();
  }
  pub fn post_update(&mut self) {
    prgl::RenderPassExecuterImpl::write_global().execute();
    prgl::Instance::flush();
    time::TimeImpl::write_global().post_update();
  }
}
