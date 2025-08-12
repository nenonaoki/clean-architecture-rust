use crate::domain::value_objects::project_id::ProjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: u64, // 楽観的ロック用
}

#[derive(Debug, thiserror::Error)]
pub enum ProjectError {
    #[error("Invalid name: {0}")]
    InvalidName(String),
}

impl Project {
    pub fn new(id: ProjectId, name: String) -> Result<Self, ProjectError> {
        // ビジネスルールの検証
        Self::validate_name(&name)?;

        let now = Utc::now();
        let user = Self {
            id,
            name,
            created_at: now,
            updated_at: now,
            version: 0,
        };

        Ok(user)
    }

    fn validate_name(name: &str) -> Result<(), ProjectError> {
        if name.trim().is_empty() {
            return Err(ProjectError::InvalidName(
                "Name cannot be empty".to_string(),
            ));
        }

        if name.len() > 100 {
            return Err(ProjectError::InvalidName("Name too long".to_string()));
        }

        // 特殊文字の検証など
        if name.chars().any(|c| c.is_control()) {
            return Err(ProjectError::InvalidName(
                "Name contains invalid characters".to_string(),
            ));
        }

        Ok(())
    }

    // 5. Getter メソッド（読み取り専用アクセス）
    pub fn id(&self) -> &ProjectId {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
    pub fn version(&self) -> u64 {
        self.version
    }
}
