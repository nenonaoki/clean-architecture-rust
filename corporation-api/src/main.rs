use actix_web::{App, HttpResponse, HttpServer, Responder, get, middleware, web};

mod application;
mod presentation;

#[get("/status")]
async fn status() -> impl Responder {
    HttpResponse::Ok().body("I'm living.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::default())
            .default_service(web::to(|| HttpResponse::NotFound()))
            .service(status)
            .configure(presentation::routes::configure_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
