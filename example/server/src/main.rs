use strattera_server::*;

fn main() {
  serve(&ServerConfig{
    port: 8080,
    host: String::from("127.0.0.1")
  });
}
