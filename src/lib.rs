//! A simple key/value store.
pub use common::{CliOperation, Response, DEFAULT_IP_ADDR};
pub use engines::{KvStore, KvsEngine, SledKvsEngine};
pub use error::{KvsError, Result};

mod common;
mod engines;
mod error;
