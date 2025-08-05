#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(Uuid);

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl UserId {
    /// 新しいUserIdを生成
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    /// 既存のUUIDからUserIdを作成
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
    
    /// 文字列からUserIdを作成（バリデーション付き）
    pub fn from_str(s: &str) -> Result<Self, UserIdError> {
        let uuid = Uuid::parse_str(s)
            .map_err(|_| UserIdError::InvalidFormat(s.to_string()))?;
        Ok(Self(uuid))
    }
    
    /// UUIDとして取得
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
    
    /// 文字列として取得
    pub fn as_str(&self) -> String {
        self.0.to_string()
    }
}
