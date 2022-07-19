//! A simple key/value store.
pub use error::{KvsError, Result};
pub use engines::{KvStore, SledKvsEngine, KvsEngine};
pub use common::{DEFAULT_IP_ADDR, CliOperation};

mod error;
mod engines;
mod common;
