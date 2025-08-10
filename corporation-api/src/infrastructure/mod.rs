use crate::domain::repositories::{project::ProjectRepository, user::UserRepository};
use std::sync::Arc;

pub mod database;
pub mod persistences;

use persistences::{project::ProjectRepositoryImpl, user::UserRepositoryImpl};

pub struct AppData {
    pub user_repository: Arc<dyn UserRepository>,
    pub project_repository: Arc<dyn ProjectRepository>,
}

pub async fn configure_app_data() -> AppData {
    let db: Arc<sea_orm::DatabaseConnection> = Arc::new(database::connect().await.unwrap());
    let user_repository: Arc<dyn UserRepository> =
        Arc::new(UserRepositoryImpl::new(Arc::clone(&db)));
    let project_repository: Arc<dyn ProjectRepository> =
        Arc::new(ProjectRepositoryImpl::new(Arc::clone(&db)));

    AppData {
        user_repository,
        project_repository,
    }
}
