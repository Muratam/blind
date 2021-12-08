use prpr_server::*;

fn main() {
  let config = ServerConfig {
    ..Default::default()
  };
  serve(&config);
}
