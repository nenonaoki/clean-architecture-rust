use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Email {
    value: String,
}

#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("Invalid email format: {0}")]
    InvalidFormat(String),
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Email {
    pub fn new(value: String) -> Result<Self, EmailError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }

    fn validate(value: &str) -> Result<(), EmailError> {
        if value.trim().is_empty() {
            return Err(EmailError::InvalidFormat(
                "Email cannot be empty".to_string(),
            ));
        }

        // 基本的なメールアドレス形式の検証
        if !value.contains('@') || !value.contains('.') {
            return Err(EmailError::InvalidFormat(
                "Invalid email format".to_string(),
            ));
        }

        let parts: Vec<&str> = value.split('@').collect();
        if parts.len() != 2 {
            return Err(EmailError::InvalidFormat(
                "Invalid email format".to_string(),
            ));
        }

        let local_part = parts[0];
        let domain_part = parts[1];

        if local_part.is_empty() || domain_part.is_empty() {
            return Err(EmailError::InvalidFormat(
                "Invalid email format".to_string(),
            ));
        }

        if !domain_part.contains('.') {
            return Err(EmailError::InvalidFormat(
                "Invalid domain format".to_string(),
            ));
        }

        Ok(())
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}
