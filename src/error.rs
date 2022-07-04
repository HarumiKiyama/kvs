use thiserror::Error;

#[derive(Debug, Error)]
pub enum KvsError {
    #[error("io error")]
    Io(#[from] io::Error),
    #[error("serde error")]
    Serde(#[from] serde_json::Error),
    #[error("Key not found")]
    KeyNotFound(),
}

pub type Result<T> = std::result::Result<T, KvsError>;

