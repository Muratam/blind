use actix_web::{*};
use actix_files::{*};
mod sandbox;

// SPA のルート階層
async fn get_index() -> impl Responder {
  HttpResponse::Ok()
    .content_type("text/html")
    .body(
r###"
<!DOCTYPE html>
<html lang="">
  <head>
    <meta charset="utf-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width,initial-scale=1.0">
    <link rel="icon" href="./favicon.ico">
    <title>aaaaa</title>
  </head>
  <body>
    <noscript>
      <strong>We're sorry but doesn't work properly without JavaScript enabled. Please enable it to continue.</strong>
    </noscript>
    <script src="./dist/js/strattera.js"></script>
  </body>
</html>
"###)
}

// やりとりは　WebSocket


#[derive(Debug)]
pub struct ServerConfig {
  pub port: i32,
  pub host: String
}


#[actix_web::main]
pub async fn serve(config: &ServerConfig) -> std::io::Result<()>  {
  let address = format!("{}:{}", config.host, config.port);
  println!("Start Strattera Server at {}", address);
  HttpServer::new(|| {
    let mut app = App::new();
    // sandbox
    app = app.route("/", web::get().to(get_index))
      .route("/sandbox/json/{echo}", web::get().to(sandbox::get_json))
      .route("/sandbox/echo", web::get().to(sandbox::get_echo));

    // file
    async fn favicon() -> Result<NamedFile> {
      Ok(NamedFile::open("./dist/favicon.ico")?)
    }
    app = app.route("/favicon.ico", web::get().to(favicon));
    let files = Files::new("/dist", "./dist");
    if cfg!(debug_assertions) {
      app = app.service(files.show_files_listing());
    } else {
      app = app.service(files);
    }
    app
  })
  .bind(address)
  .expect("Can not Start Strattera Server")
  .run()
  .await
}
