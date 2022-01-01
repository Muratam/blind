use super::*;
use crate::prgl;
pub struct Core {
  layers: Layers,
  // audio
  // etc...
}
pub struct ClientRunConfig {
  pub use_fontawesome: bool,
}
impl Default for ClientRunConfig {
  fn default() -> Self {
    Self {
      use_fontawesome: true,
    }
  }
}

impl Core {
  pub fn new(config: ClientRunConfig) -> Self {
    console_error_panic_hook::set_once();
    time::TimeImpl::initialize_global();
    rand::XorShift128::initialize_global(
      (js_sys::Date::now() % (u32::MAX & 0xffffff) as f64) as u32,
    );
    let layers = Layers::new();
    WholeScreen::initialize();
    prgl::Instance::set(layers.main_3d_context());
    prhtml::Instance::set(layers.html_layer());
    prgl::RenderPassExecuterImpl::initialize_global();
    UpdaterImpl::initialize_global();
    EventHolderImpl::initialize_global(layers.html_layer());
    if config.use_fontawesome {
      js::html::add_stylesheet_link(
        "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0-beta3/css/all.min.css",
      );
    }
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
