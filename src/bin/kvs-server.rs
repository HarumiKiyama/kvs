use log::error;
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(long, value_parser, default_value_t=String::from("127.0.0.1:4000"))]
    addr: String,
    #[clap(long, value_parser, default_value_t=String::from("default"))]
    engine: String
}

fn main() {
    let cli = Args::parse();
    println!("{}",env!("CARGO_PKG_VERSION"));
    error!("version: {}\n engine: {}\n addr: {}",env!("CARGO_PKG_VERSION"), cli.engine, cli.addr);
}
