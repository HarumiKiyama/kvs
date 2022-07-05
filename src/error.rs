use std::io;
use thiserror::Error;

/// Error type for kvs
#[derive(Debug, Error)]
pub enum KvsError {
    /// IO error
    #[error("io error")]
    Io(#[from] io::Error),
    /// serialization of deserialization error
    #[error("serde error")]
    Serde(#[from] serde_json::Error),
    /// Remove non-existence key error
    #[error("Key not found")]
    KeyNotFound,
    /// unexpected command type error
    #[error("Unsupported operation")]
    UnsupportedOperation,
}

/// Result type for kvs
pub type Result<T> = std::result::Result<T, KvsError>;

