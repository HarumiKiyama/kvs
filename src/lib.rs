//#![deny(missing_docs)]
//! A simple key/value store.

pub use error::{KvsError, Result};
pub use server::KvsServer;
pub use client::KvsClient;
pub use engines::{KvStore, SledKvsEngine, KvsEngine};

mod error;
mod client;
mod server;
mod engines;
mod common;
