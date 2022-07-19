use log::error;
use std::{
    fmt::Display,
    io::Write,
    net::{SocketAddr, TcpListener},
};

use clap::{Parser, ValueEnum};
use env_logger;

use kvs::{CliOperation, KvStore, KvsEngine, Result, DEFAULT_IP_ADDR, SledKvsEngine};

#[derive(Parser)]
#[clap(version)]
struct Args {
    #[clap(long, value_parser, default_value = DEFAULT_IP_ADDR)]
    addr: SocketAddr,
    #[clap(long, value_enum, default_value = "kvs")]
    engine: EngineChoice,
}

#[derive(ValueEnum, Clone)]
pub enum EngineChoice {
    Kvs,
    Sled,
}

impl Display for EngineChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineChoice::Kvs => write!(f, "Kvs"),
            EngineChoice::Sled => write!(f, "Sled"),
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Args::parse();
    error!(
        "version: {}\n engine: {}\n addr: {}",
        env!("CARGO_PKG_VERSION"),
        cli.engine,
        cli.addr
    );
    let mut engine = match cli.engine {
        EngineChoice::Kvs => KvStore::new(),
        EngineChoice::Sled => SledKvsEngine::new()
    }?;
    let listener = TcpListener::bind(cli.addr)?;
    for stream_res in listener.incoming() {
        let mut stream = stream_res?;
        match serde_json::from_reader(&stream)? {
            CliOperation::Set { key, value } => {
                engine.set(key, value)?;
            }
            CliOperation::Get { key } => {
                let value = engine.get(key)?.unwrap_or(String::from(""));
                write!(&mut stream, "{}", value)?;
            }
            CliOperation::Rm { key } => {
                engine.remove(key)?;
            }
        }
    }
    Ok(())
}
