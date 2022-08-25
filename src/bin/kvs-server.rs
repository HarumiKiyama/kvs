use log::error;
use serde_json::Deserializer;
use std::{
    env::current_dir,
    fmt::Display,
    fs,
    io::{BufReader, BufWriter},
    net::{SocketAddr, TcpListener},
};

use clap::{Parser, ValueEnum};
use env_logger;

use kvs::{
    KvStore, KvsEngine, NaiveThreadPool, Request, Response, Result, SledKvsEngine, ThreadPool,
    DEFAULT_IP_ADDR,
};

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

fn run_with_engine<E: KvsEngine + Send>(engine: E, addr: SocketAddr) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    let pool = NaiveThreadPool::new(1000)?;
    for stream_res in listener.incoming() {
        let engine = engine.clone();
        pool.spawn(move || {
            let stream = stream_res.unwrap();
            let reader = BufReader::new(&stream);
            let mut writer = BufWriter::new(&stream);
            let operations = Deserializer::from_reader(reader).into_iter::<Request>();
            for op in operations {
                let op = op.unwrap();
                match op {
                    Request::Set { key, value } => {
                        let value = match engine.set(key, value) {
                            Ok(..) => "ok".to_string(),
                            Err(e) => e.to_string(),
                        };
                        serde_json::to_writer(&mut writer, &Response::Set { value }).unwrap();
                    }
                    Request::Get { key } => {
                        let value = match engine.get(key).unwrap() {
                            Some(value) => value,
                            None => "Key not found".to_string(),
                        };
                        serde_json::to_writer(&stream, &Response::Get { value }).unwrap();
                    }
                    Request::Rm { key } => {
                        let value = match engine.remove(key) {
                            Ok(..) => "ok".to_string(),
                            Err(e) => e.to_string(),
                        };
                        serde_json::to_writer(&stream, &Response::Rm { value }).unwrap();
                    }
                }
            }
        })
    }
    Ok(())
}
