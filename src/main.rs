use actix_files::Files;
use actix_web::{HttpServer, App, get, Responder, HttpResponse};
use htmx_playground::routes::get_hello::get_hello;

#[get("/")]
async fn landing() -> impl Responder {
    let base = r##"
    <script src="static/htmx.min.js"></script>

    <button hx-get="/hello"
        hx-trigger="click"
        hx-swap="outerHTML"
    >
        Click Me!
    </button>"##;
    HttpResponse::Ok().body(base)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(landing).service(get_hello)
            .service(Files::new("/static", "./static"))
    })
        .bind(("127.0.0.1", 8080))?.run().await
}
