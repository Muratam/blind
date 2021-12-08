use blind_server::*;

fn main() {
  let config = ServerConfig {
    ..Default::default()
  };
  serve(&config);
}
