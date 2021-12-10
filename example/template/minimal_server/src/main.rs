use prpr_server::*;

fn main() {
  serve(&ServerConfig {
    ..Default::default()
  });
}
