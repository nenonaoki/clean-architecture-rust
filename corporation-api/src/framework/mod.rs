use log;
use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, Statement,
};
use std::env;
use std::time::Duration;

pub async fn get_db() -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    // データベース接続設定
    let protocol = env::var("DB_PROTOCOL").expect("DB_PROTOCOL must be set");
    let username = env::var("DB_USERNAME").expect("DB_USERNAME must be set");
    let password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let host = env::var("DB_HOST").expect("DB_HOST must be set");
    let port = env::var("DB_PORT").expect("DB_PORT must be set");
    let database = env::var("DB_NAME").expect("DB_NAME must be set");
    let schema = env::var("DB_SCHEME").expect("DB_SCHEME must be set");

    let mut opt = ConnectOptions::new(format!(
        "{}://{}:{}@{}:{}/{}?currentSchema={}",
        protocol, username, password, host, port, database, schema
    ));
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("project"); // Setting default PostgreSQL schema
    let db = Database::connect(opt).await?;

    let test = db
        .query_all(Statement::from_string(
            DatabaseBackend::Postgres,
            "SELECT * FROM project.projects LIMIT 10".to_string(),
        ))
        .await;

    println!("test: {:?}", test.unwrap());

    Ok(db)
}
