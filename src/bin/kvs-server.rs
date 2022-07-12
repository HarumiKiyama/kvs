use std::fmt::Display;
use std::net::SocketAddr;

use log::error;
use clap::{Parser, ValueEnum};
use kvs::DEFAULT_IP_ADDR;

#[derive(Parser)]
#[clap(version)]
struct Args {
    #[clap(long, value_parser, default_value = DEFAULT_IP_ADDR)]
    addr: SocketAddr,
    #[clap(long, value_enum)]
    engine: Option<EngineChoice>,
}

#[derive(ValueEnum, Clone)]
pub enum EngineChoice {
    Kvs,
    Sled,
}


impl Display for EngineChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineChoice::Kvs=> write!(f, "Kvs"),
            EngineChoice::Sled=> write!(f, "Sled")
        }
    }
}


fn main() {
    let cli = Args::parse();
    error!("test");
    error!("version: {}\n engine: {}\n addr: {}",env!("CARGO_PKG_VERSION"), cli.engine.unwrap_or(EngineChoice::Kvs), cli.addr);
}
