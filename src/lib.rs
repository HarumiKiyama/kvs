//#![deny(missing_docs)]
//! A simple key/value store.

pub use error::{KvsError, Result};
pub use server::KvsServer;
pub use client::KvsClient;
pub use engines::{KvStore, SledKvsEngine, KvsEngine};
pub use common::DEFAULT_IP_ADDR;

mod error;
mod client;
mod server;
mod engines;
mod common;
