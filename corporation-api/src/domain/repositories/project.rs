use crate::domain::entities::project::Project;
use crate::domain::value_objects::project_id::ProjectId;
use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum FindByIdError {
    #[error("Project not found")]
    NotFound,

    #[error("unknown error")]
    Unknown,
}

#[async_trait]

pub trait ProjectRepository: Send + Sync {
    async fn find_by_id(&self, id: ProjectId) -> Result<Option<Project>, FindByIdError>;
}
