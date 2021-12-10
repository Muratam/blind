use crate::js;
mod core;
mod layers;
pub use self::core::Core;

pub trait System {
  fn new(core: &Core) -> Self;
  fn update(&mut self, core: &Core);
}
pub fn run<T: 'static + System>() {
  let mut core = Core::new();
  let mut system = T::new(&core);
  js::start_animation_frame_loop(Box::new(move || {
    core.update();
    system.update(&core);
  }))
}
