use super::*;
use crate::js;
mod core;
pub use self::core::Core;
mod layers;
use self::layers::Layers;
pub mod log;
mod time;
pub use time::Time;
mod updater;
pub use updater::*;
mod event;
pub use event::*;
pub mod input;

pub fn run(f: fn()) {
  let mut core = Core::new();
  f();
  js::start_animation_frame_loop(Box::new(move || {
    core.pre_update();
    core.update();
    core.post_update();
  }))
}
