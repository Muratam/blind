use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[actix_web::get("/")]
async fn get_index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::get("/sandbox/json/{echo}")]
async fn get_json_echo(echo: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(format!("{}", echo))
}

#[actix_web::get("/sandbox/echo")]
async fn get_echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let host = "127.0.0.1";
    let port = 8088;
    let address = format!("{}:{}", host, port);
    println!("Start Rust Server at {}", address);
    HttpServer::new(|| {
        App::new()
            .service(get_index)
            .service(get_echo)
            .service(get_json_echo)
    })
    .bind(address)?
    .run()
    .await
}
