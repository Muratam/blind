use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[actix_web::get("/")]
async fn get_index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::get("/get/{echo}")]
async fn get_echo(echo: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(format!("{}", echo))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let host = "127.0.0.1";
    let port = 8088;
    let address = format!("{}:{}", host, port);
    println!("Start Rust Server at {}", address);
    HttpServer::new(|| App::new().service(get_index).service(get_echo))
        .bind(address)?
        .run()
        .await
}
