use serde::Deserialize;
use serde_json::de::Deserializer;
use std::{
    io::{BufReader, BufWriter, Write},
    net::{SocketAddr, TcpStream},
    time::Duration,
};

use clap::{Parser, Subcommand};

use kvs::{Request, Response, Result, DEFAULT_IP_ADDR};

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
            let output = Response::deserialize(&mut reader)?;
            println!("{}", output);
        }
        _ => {}
    }
    Ok(())
}
