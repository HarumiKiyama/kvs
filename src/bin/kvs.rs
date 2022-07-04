use std::{process::exit, env::current_dir};

use clap::{Parser, Subcommand};
use kvs::KvStore;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
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
        Some(Commands::Set { key, value }) => {
            kv_store.set(key.to_string(), value.to_string()).unwrap();
        }
        Some(Commands::Get { key }) => match kv_store.get(key.to_string()) {
            Ok(Some(v)) => println!("{}", v),
            _ => {println!("Key not found")},
        },
        Some(Commands::Rm { key }) => match kv_store.remove(key.to_string()) {
            Ok(_) => (),
            _ => {
                println!("Key not found");
                exit(1);
            },
        },
        None => {
            unimplemented!();
        }
    }
}
