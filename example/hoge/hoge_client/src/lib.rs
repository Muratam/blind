use prpr_client::*;

#[entry_point(start)]
pub fn start() {
  system::run(sample::sample_world);
}
