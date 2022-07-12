use std::net::SocketAddr;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Command>,

    #[clap(long, value_parser)]
    addr: Option<SocketAddr>,
}

#[derive(Subcommand)]
enum Command {
    Set {
        #[clap(value_parser)]
        key: String,
        #[clap(value_parser)]
        value: String,
    },
    Get {
        #[clap(value_parser)]
        key: String,
    },
    Rm {
        #[clap(value_parser)]
        key: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Command::Set { key, value }) => {
            todo!();
        }
        Some(Command::Get { key }) => {
            todo!();
        }
        Some(Command::Rm { key }) => {
            todo!();
        }
        None => {
            unimplemented!();
        }
    }
}
