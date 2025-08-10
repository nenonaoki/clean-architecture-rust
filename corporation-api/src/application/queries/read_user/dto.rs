use crate::domain::repositories::user::FindByIdError;

#[derive(Debug)]
pub struct ReadUserInput {
    pub user_id: u32,
}

#[derive(Debug)]
pub struct ReadUserOutput {
    pub user_id: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum ReadUserError {
    #[error("user not found")]
    NotFound,

    #[error("unknown error")]
    Unknown,
}

impl From<FindByIdError> for ReadUserError {
    fn from(error: FindByIdError) -> Self {
        match error {
            FindByIdError::NotFound => ReadUserError::NotFound,
            FindByIdError::Unknown => ReadUserError::Unknown,
        }
    }
}
