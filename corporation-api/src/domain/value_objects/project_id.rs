use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProjectId(u32);

#[derive(Debug, thiserror::Error)]
pub enum ProjectIdError {
    #[error("Invalid project ID format: {0}")]
    InvalidFormat(String),
}

impl Default for ProjectId {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectId {
    /// 新しいUserIdを生成
    pub fn new() -> Self {
        Self(0) // デフォルト値として0を使用
    }

    /// 既存のu32からUserIdを作成
    pub fn from_u32(id: u32) -> Self {
        Self(id)
    }

    /// 文字列からUserIdを作成（バリデーション付き）
    pub fn from_str(s: &str) -> Result<Self, ProjectIdError> {
        let id = s
            .parse::<u32>()
            .map_err(|_| ProjectIdError::InvalidFormat(s.to_string()))?;
        Ok(Self(id))
    }

    /// u32として取得
    pub fn as_u32(&self) -> u32 {
        self.0
    }

    /// 文字列として取得
    pub fn as_str(&self) -> String {
        self.0.to_string()
    }
}
