use crate::domain::entities::project::Project;
use crate::domain::repositories::project::{FindByIdError, ProjectRepository};
use crate::domain::value_objects::project_id::ProjectId;
use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// データベースのユーザーテーブルに対応するエンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectModel {
    pub id: u32,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: u64,
}

impl ProjectModel {
    fn to_domain(self) -> Result<Project, FindByIdError> {
        let project_id = ProjectId::from_u32(self.id);

        // Project::newは検証を行うため、既存のデータの場合は直接構築
        let project: Project = Project {
            id: project_id,
            name: self.name,
            created_at: self.created_at,
            updated_at: self.updated_at,
            version: self.version,
        };

        Ok(project)
    }
}

pub struct ProjectRepositoryImpl {
    db: Arc<DatabaseConnection>,
}

impl ProjectRepositoryImpl {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ProjectRepository for ProjectRepositoryImpl {
    async fn find_by_id(&self, id: ProjectId) -> Result<Option<Project>, FindByIdError> {
        let project_id = id.as_u32();
        let project_model = ProjectModel {
            id: project_id,
            name: "Test Name".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            version: 0u64,
        };

        let project = project_model.to_domain()?;
        Ok(Some(project))
    }
}
