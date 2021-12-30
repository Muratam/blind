use super::*;
use crate::js;
mod core;
pub use self::core::{ClientRunConfig, Core};
mod layers;
use self::layers::Layers;
pub mod log;
mod time;
pub use time::Time;
mod updater;
pub use updater::*;
mod event;
pub use event::*;
mod screen;
pub use screen::*;
pub mod input;

pub fn run(f: fn(), config: Option<ClientRunConfig>) {
  let mut core = Core::new(config.unwrap_or_default());
  f();
  js::start_animation_frame_loop(Box::new(move || {
    core.pre_update();
    core.update();
    core.post_update();
  }))
}
