use std::fmt::Display;
use std::net::SocketAddr;

use log::error;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[clap(version)]
struct Args {
    #[clap(long, value_parser)]
    addr: Option<SocketAddr>,
    #[clap(long, value_enum)]
    engine: Option<EngineChoice>,
}

#[derive(ValueEnum, Clone)]
pub enum EngineChoice{
    KvsEngine,
    SledEngine
}



impl Display for EngineChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineChoice::KvsEngine  => write!(f, "KvsEngine"),
            EngineChoice::SledEngine => write!(f, "SledEngine")
        }
    }

}


fn main() {
    let cli = Args::parse();
    error!("version: {}\n engine: {}\n addr: {}",env!("CARGO_PKG_VERSION"), cli.engine.unwrap_or(EngineChoice::KvsEngine), cli.addr.unwrap_or("192.168.111.1:80".parse().unwrap()));
}
