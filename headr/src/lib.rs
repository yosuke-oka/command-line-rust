use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
pub struct Config {
    /// Input files
    #[arg(default_value = "-")]
    files: Vec<String>,

    /// Number of lines
    #[arg(
        short = 'n',
        long = "lines",
        conflicts_with = "bytes",
        default_value = "10"
    )]
    lines: usize,

    /// Number of bytes
    #[arg(short = 'c', long = "bytes", conflicts_with = "lines")]
    bytes: Option<usize>,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    Ok(Config::parse())
}
