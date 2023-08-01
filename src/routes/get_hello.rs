use actix_web::{HttpServer, App, get, Responder, HttpResponse};

#[get("/hello")]
pub async fn get_hello() -> impl Responder {
    HttpResponse::Ok().body("<p>Hello World</p>")
}
