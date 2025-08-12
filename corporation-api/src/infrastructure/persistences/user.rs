use crate::domain::entities::user::User;
use crate::domain::repositories::user::{FindByIdError, UserRepository};
use crate::domain::value_objects::email::Email;
use crate::domain::value_objects::user_id::UserId;
use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// データベースのユーザーテーブルに対応するエンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserModel {
    pub id: u32,
    pub email: String,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: u64,
}

impl UserModel {
    fn to_domain(self) -> Result<User, FindByIdError> {
        let user_id = UserId::from_u32(self.id);
        let email = Email::new(self.email).map_err(|_| FindByIdError::Unknown)?;

        // User::newは検証を行うため、既存のデータの場合は直接構築
        let user = User {
            id: user_id,
            email,
            name: self.name,
            created_at: self.created_at,
            updated_at: self.updated_at,
            version: self.version,
        };

        Ok(user)
    }
}

pub struct UserRepositoryImpl {
    db: Arc<DatabaseConnection>,
}

impl UserRepositoryImpl {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, FindByIdError> {
        let user_id = id.as_u32();

        // let test = db
        //     .query_all(Statement::from_string(
        //         DatabaseBackend::Postgres,
        //         "SELECT * FROM project.projects LIMIT 10".to_string(),
        //     ))
        //     .await;

        // println!("test: {:?}", test.unwrap());

        // let result = self
        //     .db
        //     .query_all(Statement::from_sql_and_values(
        //         DatabaseBackend::Postgres,
        //         r#"
        //     SELECT * FROM project.projects LIMIT 10
        //     "#,
        //         vec![Value::Int(Some(user_id as i32))],
        //     ))
        //     .await?
        //     .map_err(|_| FindByIdError::Unknown)?;

        // if result.is_empty() {
        //     return Ok(None);
        // }

        // // 結果をUserModelに変換
        // let row = &result[0];
        // let user_model = UserModel {
        //     id: row
        //         .try_get::<u32>("", "id")
        //         .map_err(|_| FindByIdError::Unknown)?,
        //     email: row
        //         .try_get::<String>("", "email")
        //         .map_err(|_| FindByIdError::Unknown)?,
        //     name: row
        //         .try_get::<String>("", "name")
        //         .map_err(|_| FindByIdError::Unknown)?,
        //     created_at: row
        //         .try_get::<chrono::DateTime<chrono::Utc>>("", "created_at")
        //         .map_err(|_| FindByIdError::Unknown)?,
        //     updated_at: row
        //         .try_get::<chrono::DateTime<chrono::Utc>>("", "updated_at")
        //         .map_err(|_| FindByIdError::Unknown)?,
        //     version: row
        //         .try_get::<u64>("", "version")
        //         .map_err(|_| FindByIdError::Unknown)?,
        // };

        let user_model = UserModel {
            id: user_id,
            email: "test@example.com".to_string(),
            name: "Test Name".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            version: 0u64,
        };

        let user = user_model.to_domain()?;
        Ok(Some(user))
    }
}
