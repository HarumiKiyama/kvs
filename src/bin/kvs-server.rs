use log::error;
use std::env::current_dir;
use std::{
    fmt::Display,
    fs,
    net::{SocketAddr, TcpListener},
};

use clap::{Parser, ValueEnum};
use env_logger;

use kvs::{CliOperation, KvStore, KvsEngine, Response, Result, SledKvsEngine, DEFAULT_IP_ADDR};

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
            EngineChoice::Kvs => write!(f, "Kvs"),
            EngineChoice::Sled => write!(f, "Sled"),
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Args::parse();
    let mut engine = EngineChoice::Kvs;
    let former_engine = fs::read_to_string("engine").unwrap_or(String::from(""));
    match former_engine.as_str() {
        "kvs" => match &cli.engine {
            Some(EngineChoice::Kvs) | None => {
                engine = EngineChoice::Kvs;
            }
            _ => {
                error!(
                    "error engine: former_engine: {}, selected engine kvs",
                    former_engine
                );
            }
        },
        "sled" => match &cli.engine {
            Some(EngineChoice::Sled) | None => {
                engine = EngineChoice::Sled;
            }
            _ => {
                error!(
                    "error engine: former_engine: {}, selected engine kvs",
                    former_engine
                );
            }
        },
        "" => match &cli.engine {
            Some(EngineChoice::Kvs) | None => {
                engine = EngineChoice::Kvs;
                fs::write("engine", "kvs")?;
            }
            Some(EngineChoice::Sled) => {
                engine = EngineChoice::Sled;
                fs::write("engine", "sled")?;
            }
        },
        _ => {
            error!("wrong engine name written in file");
        }
    }

    error!(
        "version: {}\n engine: {}\n addr: {}",
        env!("CARGO_PKG_VERSION"),
        engine,
        cli.addr
    );
    match engine {
        EngineChoice::Kvs => run_with_engine(KvStore::open(current_dir()?)?, cli.addr),
        EngineChoice::Sled => run_with_engine(SledKvsEngine::open(current_dir()?)?, cli.addr),
    }
}

fn run_with_engine(mut engine: impl KvsEngine, addr: SocketAddr) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    for stream_res in listener.incoming() {
        let stream = stream_res?;
        match serde_json::from_reader(&stream)? {
            CliOperation::Set { key, value } => {
                engine.set(key, value)?;
                serde_json::to_writer(
                    &stream,
                    &Response::Set {
                        value: String::new(),
                    },
                )?;
            }
            CliOperation::Get { key } => {
                let value = engine.get(key)?.unwrap_or(String::from(""));
                serde_json::to_writer(&stream, &Response::Get { value })?;
            }
            CliOperation::Rm { key } => {
                engine.remove(key)?;
                serde_json::to_writer(
                    &stream,
                    &Response::Rm {
                        value: String::new(),
                    },
                )?;
            }
        }
    }
    Ok(())
}
