use crate::domain::repositories::user::FindByIdError;
use thiserror::Error;

#[derive(Debug)]
pub struct ReadUserInput {
    pub user_id: u32,
}

#[derive(Debug)]
pub struct ReadUserOutput {
    pub user_id: u32,
}

#[derive(Error, Debug)]
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
