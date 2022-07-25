use env_logger;
use log::error;
use serde::Deserialize;
use serde_json::de::Deserializer;
use std::{
    io::{BufReader, BufWriter, Write},
    net::{SocketAddr, TcpStream},
    time::Duration,
};

use clap::{Parser, Subcommand};

use kvs::{KvsError, Request, Response, Result, DEFAULT_IP_ADDR};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Set {
        #[clap(value_parser)]
        key: String,
        #[clap(value_parser)]
        value: String,
        #[clap(long, value_parser, default_value = DEFAULT_IP_ADDR)]
        addr: SocketAddr,
    },
    Get {
        #[clap(value_parser)]
        key: String,
        #[clap(long, value_parser, default_value = DEFAULT_IP_ADDR)]
        addr: SocketAddr,
    },
    Rm {
        #[clap(value_parser)]
        key: String,
        #[clap(long, value_parser, default_value = DEFAULT_IP_ADDR)]
        addr: SocketAddr,
    },
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();
    match cli.command {
        Some(Command::Set { key, value, addr }) => run(Request::Set { key, value }, addr),
        Some(Command::Get { key, addr }) => run(Request::Get { key }, addr),
        Some(Command::Rm { key, addr }) => run(Request::Rm { key }, addr),
        None => {
            unimplemented!();
        }
    }
}

fn run(op: Request, addr: SocketAddr) -> Result<()> {
    let stream = TcpStream::connect(addr)?;
    stream.set_read_timeout(Some(Duration::from_secs(3)))?;
    let mut reader = Deserializer::from_reader(BufReader::new(&stream));
    let mut writer = BufWriter::new(&stream);
    serde_json::to_writer(&mut writer, &op)?;
    writer.flush()?;
    match op {
        Request::Get { .. } => {
            if let Response::Get { value } = Response::deserialize(&mut reader)? {
                println!("{}", value);
            };
        }
        Request::Rm { .. } => {
            if let Response::Rm { value } = Response::deserialize(&mut reader)? {
                if value.as_str() != "ok" {
                    error!("{}", value);
                    return Err(KvsError::KeyNotFound);
                }
            }
        }
        _ => {}
    }
    Ok(())
}
