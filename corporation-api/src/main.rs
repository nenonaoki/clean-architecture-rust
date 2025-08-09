use actix_web::{App, HttpResponse, HttpServer, Responder, get, middleware, web};
use dotenv::dotenv;
use std::sync::Arc;
use tracing;
use tracing_subscriber;

mod application;
mod domain;
mod infrastructure;
mod presentation;

#[get("/status")]
async fn status() -> impl Responder {
    HttpResponse::Ok().body("I'm living.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // https://qiita.com/Yukimura127/items/c3f199bbdfb0fee34015
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let db = Arc::new(infrastructure::database::get_db().await.unwrap());
    let user_repository: Arc<dyn domain::repositories::user::UserRepository> = Arc::new(
        infrastructure::persistences::user::UserRepositoryImpl::new(Arc::clone(&db)),
    );
    // let project_repository: Arc<dyn domain::repositories::project::ProjectRepository> = Arc::new(
    //     infrastructure::persistences::project::ProjectRepositoryImpl::new(Arc::clone(&db)),
    // );

    // let test = user_repository.clone()

    // サーバー起動
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_repository.clone()))
            // .app_data(web::Data::new(Arc::clone(&project_repository)))
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
