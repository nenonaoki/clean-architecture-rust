pub struct Input {
    pub user_name: String,
}

pub struct Output {
    pub user_name: String,
}

#[derive(Debug)]
pub enum ErrorStatus {
    Unknown,
}

#[derive(Debug)]
pub struct Error {
    pub status: ErrorStatus,
}
