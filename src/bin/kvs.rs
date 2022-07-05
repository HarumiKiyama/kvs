use std::{env::current_dir, process::exit};

use clap::{Parser, Subcommand};
use kvs::{KvsError, KvStore};

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
    let dir = current_dir().unwrap();
    let mut kv_store = KvStore::open(dir).unwrap();
    match &cli.command {
        Some(Command::Set { key, value }) => {
            kv_store.set(key.to_string(), value.to_string()).unwrap();
        }
        Some(Command::Get { key }) => match kv_store.get(key.to_string()) {
            Ok(Some(v)) => println!("{}", v),
            _ => {
                println!("Key not found")
            }
        },
        Some(Command::Rm { key }) => match kv_store.remove(key.to_string()) {
            Ok(_) => (),
            Err(KvsError::KeyNotFound) => {
                println!("Key not found");
                exit(1)
            }
            Err(e) => {
                println!("{:?}", e);
                exit(1);
            }
        },
        None => {
            unimplemented!();
        }
    }
}
