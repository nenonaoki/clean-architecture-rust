use super::dto::{ReadUserError, ReadUserInput, ReadUserOutput};
use crate::domain::repositories::user::UserRepository;
use crate::domain::value_objects::user_id::UserId;
use std::sync::Arc;

pub struct ReadUserInteractor {
    user_repository: Arc<dyn UserRepository>,
}

impl ReadUserInteractor {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, input: ReadUserInput) -> Result<ReadUserOutput, ReadUserError> {
        let user_id = UserId::from_u32(input.user_id);
        let user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or(ReadUserError::NotFound)?;

        let output = ReadUserOutput {
            user_id: user.id().as_u32(),
        };

        Ok(output)
    }
}
