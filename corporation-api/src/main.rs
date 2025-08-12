use actix_web::{App, HttpResponse, HttpServer, middleware, web};
use dotenv::dotenv;

mod application;
mod domain;
mod infrastructure;
mod presentation;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // https://qiita.com/Yukimura127/items/c3f199bbdfb0fee34015
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    // Configure App Data (DI)
    let app_data_config = infrastructure::configure_app_data().await;
    let app_data = web::Data::new(app_data_config);

    // Launch server
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .default_service(web::to(HttpResponse::NotFound))
            .configure(presentation::routes::configure_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
