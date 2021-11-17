use actix_web::{*};
use actix_files::{*};
use crate::const_params::{*};
use crate::html;

// SPA のルート階層
async fn get_index() -> impl Responder {
  html::respond_html()
}

// テスト用１
async fn get_sandbox_json(echo: web::Path<String>) -> impl Responder {
  HttpResponse::Ok().json(format!("{}", echo))
}

// テスト用２
async fn get_sandbox_echo(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}


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
      .route("/sandbox/json/{echo}", web::get().to(get_sandbox_json))
      .route("/sandbox/echo", web::get().to(get_sandbox_echo));
    // file
    let files = Files::new(format!("/{}", RESOURCE_ROOT_DIR_NAME).as_str(), "./dist");
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
