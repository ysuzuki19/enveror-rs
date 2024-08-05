#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io::Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("serde_json::Error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("InvalidConfig: {0}")]
    InvalidConfig(String),

    #[error("{0}")]
    Custom(String),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
