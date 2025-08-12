use std::sync::Arc;

pub mod database;
pub mod persistences;

use persistences::{Container, project::ProjectRepositoryImpl, user::UserRepositoryImpl};

pub struct AppData {
    pub container: Container,
}

pub async fn configure_app_data() -> AppData {
    let db: Arc<sea_orm::DatabaseConnection> = Arc::new(database::connect().await.unwrap());
    let container = Container::new();
    let _ = container.register("UserRepository", UserRepositoryImpl::new(Arc::clone(&db)));
    let _ = container.register(
        "ProjectRepository",
        ProjectRepositoryImpl::new(Arc::clone(&db)),
    );

    AppData { container }
}
