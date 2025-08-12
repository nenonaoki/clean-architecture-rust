use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

pub mod database;
pub mod persistences;

use persistences::{project::ProjectRepositoryImpl, user::UserRepositoryImpl};

#[derive(Debug, thiserror::Error)]
pub enum DIError {
    #[error("dependency not found")]
    NotFound,

    #[error("unknown error")]
    Unknown,
}

pub struct Container {
    dependencies: RwLock<HashMap<String, Arc<dyn Any + Send + Sync>>>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            dependencies: RwLock::new(HashMap::new()),
        }
    }

    pub fn register<T>(&self, name: &str, dependency: T) -> Result<(), DIError>
    where
        T: 'static + Send + Sync,
    {
        let mut dependencies = self.dependencies.write().unwrap();

        dependencies.insert(name.to_string(), Arc::new(dependency));
        Ok(())
    }

    pub fn resolve<T>(&self, name: &str) -> Result<Arc<T>, DIError>
    where
        T: 'static + Send + Sync,
    {
        let dependencies = self.dependencies.read().unwrap();
        let dependency = dependencies.get(name).ok_or_else(|| DIError::NotFound)?;

        dependency
            .clone()
            .downcast::<T>()
            .map_err(|_| DIError::Unknown)
    }
}

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
