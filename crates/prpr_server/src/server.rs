use crate::const_params::*;
use crate::html;
use actix_files::*;
use actix_web::*;

#[derive(Debug)]
pub struct ServerConfig {
  pub port: i32,
  pub host: String,
}

impl Default for ServerConfig {
  fn default() -> Self {
    Self {
      port: 7272,
      host: String::from("127.0.0.1"),
    }
  }
}

// SPA のルート階層
async fn get_index() -> impl Responder {
  // todo: シーン毎対応して複数階層のページが閲覧できるように
  let config = html::WebPageConfig {
    ..Default::default()
  };
  html::respond_html(&config)
}

#[actix_web::main]
pub async fn serve(config: &ServerConfig) {
  let address = format!("{}:{}", config.host, config.port);
  println!("Start prpr Server at {}", address);
  let _ = HttpServer::new(|| {
    let mut app = App::new();
    // scenes
    app = app.route("/", web::get().to(get_index));
    // file
    let files = Files::new(format!("/{}", RESOURCE_ROOT_DIR_NAME).as_str(), "./dist");
    if cfg!(debug_assertions) {
      app = app.service(files.show_files_listing());
    } else {
      app = app.service(files);
    }
    // sandbox(テスト用)
    async fn get_sandbox_json(echo: web::Path<String>) -> impl Responder {
      HttpResponse::Ok().json(format!("{}", echo))
    }
    async fn get_sandbox_echo(req_body: String) -> impl Responder {
      HttpResponse::Ok().body(req_body)
    }
    app = app
      .route("/sandbox/json/{echo}", web::get().to(get_sandbox_json))
      .route("/sandbox/echo", web::get().to(get_sandbox_echo));
    // final
    app
  })
  .bind(address)
  .expect("Can not Start prpr Server")
  .run()
  .await;
}
