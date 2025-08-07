use actix_web::{App, HttpResponse, HttpServer, Responder, get, middleware, web};
use dotenv::dotenv;
use tracing;
use tracing_subscriber;

mod application;
mod framework;
mod presentation;

#[get("/status")]
async fn status() -> impl Responder {
    HttpResponse::Ok().body("I'm living.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let db = framework::get_db().await.unwrap();
    println!("db: {:?}", db);

    // サーバー起動
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .default_service(web::to(|| HttpResponse::NotFound()))
            .service(status)
            .configure(presentation::routes::configure_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
