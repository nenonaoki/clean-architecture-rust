#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    id: UserId,
    email: Email,
    name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    version: u64, // 楽観的ロック用
}

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("Invalid name: {0}")]
    InvalidName(String),
    
    #[error("User is already active")]
    AlreadyActive,
    
    #[error("User is already deleted")]
    AlreadyDeleted,
    
    #[error("Cannot activate deleted user")]
    CannotActivateDeletedUser,
    
    #[error("Cannot deactivate deleted user")]
    CannotDeactivateDeletedUser,
    
    #[error("Cannot suspend deleted user")]
    CannotSuspendDeletedUser,
    
    #[error("Cannot change email of deleted user")]
    CannotChangeEmailOfDeletedUser,
    
    #[error("No events provided for reconstruction")]
    NoEventsProvided,
    
    #[error("Invalid event sequence")]
    InvalidEventSequence,
}

impl User {
    pub fn new(id: UserId, email: Email, name: String) -> Result<Self, UserError> {
        // ビジネスルールの検証
        Self::validate_name(&name)?;
        
        let now = Utc::now();
        let mut user = Self {
            id: id.clone(),
            email: email.clone(),
            name,
            created_at: now,
            updated_at: now,
            version: 0,
        };
        
        Ok(user)
    }
    
    fn validate_name(name: &str) -> Result<(), UserError> {
        if name.trim().is_empty() {
            return Err(UserError::InvalidName("Name cannot be empty".to_string()));
        }
        
        if name.len() > 100 {
            return Err(UserError::InvalidName("Name too long".to_string()));
        }
        
        // 特殊文字の検証など
        if name.chars().any(|c| c.is_control()) {
            return Err(UserError::InvalidName("Name contains invalid characters".to_string()));
        }
        
        Ok(())
    }
    
    // 5. Getter メソッド（読み取り専用アクセス）
    pub fn id(&self) -> &UserId { &self.id }
    pub fn email(&self) -> &Email { &self.email }
    pub fn name(&self) -> &str { &self.name }
    pub fn created_at(&self) -> DateTime<Utc> { self.created_at }
    pub fn updated_at(&self) -> DateTime<Utc> { self.updated_at }
    pub fn version(&self) -> u64 { self.version }
}