use crate::domain::repositories;
use std::sync::Arc;

pub mod database;
pub mod persistences;

pub struct AppData {
    pub user_repository: Arc<dyn repositories::user::UserRepository>,
    pub project_repository: Arc<dyn repositories::project::ProjectRepository>,
}

pub async fn configure_app_data() -> AppData {
    let db: Arc<sea_orm::DatabaseConnection> = Arc::new(database::connect().await.unwrap());
    let user_repository: Arc<dyn repositories::user::UserRepository> =
        Arc::new(persistences::user::UserRepositoryImpl::new(Arc::clone(&db)));
    let project_repository: Arc<dyn repositories::project::ProjectRepository> = Arc::new(
        persistences::project::ProjectRepositoryImpl::new(Arc::clone(&db)),
    );

    AppData {
        user_repository,
        project_repository,
    }
}
