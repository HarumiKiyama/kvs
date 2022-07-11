use log::error;
use clap::{self, Parser, ValueEnum, ARG, arg_enum};

#[derive(Parser)]
#[clap(version)]
struct Args {
    #[clap(long, value_parser)]
    addr: Option<String>,
    #[clap(long, value_parser)]
    engine: Option<EngineChoice>,
}
arg_enum! {
    #[derive(Debug)]
    pub enum EngineChoice{
        KvsEngine,
        SledEngine
    }

}


fn main() {
    let cli = Args::parse();
    error!("version: {}\n engine: {}\n addr: {}",env!("CARGO_PKG_VERSION"), cli.engine, cli.addr);
}
