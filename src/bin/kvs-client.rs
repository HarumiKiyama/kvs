use std::{net::{SocketAddr, TcpStream}, io::Read};

use clap::{Parser, Subcommand};

use kvs::{CliOperation, DEFAULT_IP_ADDR, Result};

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

fn main() -> Result<()>{
    let cli = Cli::parse();
    match cli.command {
        Some(Command::Set { key, value, addr }) => {
            run(
                CliOperation::Set {
                    key,
                    value,
                },
                addr,
            )
        }
        Some(Command::Get { key, addr }) => {
            run(
                CliOperation::Get {
                    key,
                },
                addr,
            )
        }
        Some(Command::Rm { key, addr }) => {
            run(
                CliOperation::Rm {
                    key,
                },
                addr,
            )
        }
        None => {
            unimplemented!();
        }
    }
}

fn run(op: CliOperation, addr: SocketAddr) -> Result<()>{
    let mut stream = TcpStream::connect(addr)?;
    serde_json::to_writer(&mut stream, &op)?;
    let mut output = String::new();
    stream.read_to_string(&mut output)?;
    println!("{}", output);
    Ok(())
}
