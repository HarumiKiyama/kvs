//#![deny(missing_docs)]
//! A simple key/value store.

pub use error::{KvsError, Result};
pub use server::KvStore;

mod error;
mod server;
