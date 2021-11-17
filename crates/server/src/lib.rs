use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn get_index() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

async fn get_json_echo(echo: web::Path<String>) -> impl Responder {
  HttpResponse::Ok().json(format!("{}", echo))
}

async fn get_echo(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}

#[actix_rt::main]
pub async fn serve(port: i32) -> std::io::Result<()>  {
  let host = "127.0.0.1";
  let address = format!("{}:{}", host, port);
  println!("Start Rust Server at {}", address);
  HttpServer::new(|| {
    App::new()
      .route("/", web::get().to(get_index))
      .route("/sandbox/json/{echo}", web::get().to(get_json_echo))
      .route("/sandbox/echo", web::get().to(get_echo))
  })
  .bind(address)?
  .run()
  .await
}
