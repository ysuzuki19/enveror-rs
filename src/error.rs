#[derive(Debug, thiserror::Error)]
pub enum EnverorError {
    #[error("io::Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("InvalidConfig: {0}")]
    InvalidConfig(String),

    #[error("{0}")]
    Custom(String),
}

pub type EnverorResult<T> = std::result::Result<T, EnverorError>;
