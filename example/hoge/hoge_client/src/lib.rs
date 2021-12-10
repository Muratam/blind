use prpr_client::*;

#[prpr_client::entry_point(start)]
pub fn start() {
  prpr_client::world::create();
}
