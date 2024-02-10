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

    /// Number lines
    #[arg(short = 'n', long = "number", conflicts_with = "number_nonblank_lines")]
    number_lines: bool,

    /// Number nonblank lines
    #[arg(short = 'b', long = "number-nonblank", conflicts_with = "number_lines")]
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for file in config.files {
        match open(&file) {
            Err(e) => eprintln!("Failed to Open {}: {}", file, e),
            Ok(buf) => {
                let mut line_number = 1;
                for line in buf.lines() {
                    let line = line?;
                    if config.number_lines {
                        println!("{:>6}\t{}", line_number, line);
                        line_number += 1;
                    } else if config.number_nonblank_lines {
                        if line.is_empty() {
                            println!("");
                        } else {
                            println!("{:>6}\t{}", line_number, line);
                            line_number += 1;
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    Ok(Config::parse())
}

fn open(file: &str) -> MyResult<Box<dyn BufRead>> {
    match file {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file)?))),
    }
}
