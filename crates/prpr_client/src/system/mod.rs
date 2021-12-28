use super::*;
use crate::js;
mod core;
pub use self::core::Core;
mod layers;
use self::layers::Layers;
pub mod log;
mod time;
pub use time::Time;

pub trait System {
  fn new(core: &Core) -> Self;
  fn update(&mut self, core: &Core);
}
pub fn run<T: 'static + System>() {
  let mut core = Core::new();
  let mut system = T::new(&core);
  js::start_animation_frame_loop(Box::new(move || {
    core.pre_update();
    system.update(&core);
    core.post_update();
  }))
}
