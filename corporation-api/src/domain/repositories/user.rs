use crate::domain::entities::user::User;
use crate::domain::value_objects::user_id::UserId;
use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum FindByIdError {
    #[error("User not found")]
    NotFound,

    #[error("unknown error")]
    Unknown,
}

// #[derive(Debug, thiserror::Error)]
// pub enum SaveError {
//     #[error("Database error: {0}")]
//     Database(String),
//     #[error("Validation error: {0}")]
//     Validation(String),
// }

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, FindByIdError>;
    // async fn save(&self, user: User) -> Result<(), SaveError>;
}
