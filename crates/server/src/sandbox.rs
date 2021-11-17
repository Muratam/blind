use actix_web::{*};

// テスト用１
pub async fn get_json(echo: web::Path<String>) -> impl Responder {
  HttpResponse::Ok().json(format!("{}", echo))
}

// テスト用２
pub async fn get_echo(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}
