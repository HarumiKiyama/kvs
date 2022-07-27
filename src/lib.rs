//! A simple key/value store.
pub use common::{Request, Response, DEFAULT_IP_ADDR};
pub use engines::{KvStore, KvsEngine, SledKvsEngine};
pub use error::{KvsError, Result};
pub use thread_pool::ThreadPool;

mod common;
mod engines;
mod error;
mod thread_pool;
