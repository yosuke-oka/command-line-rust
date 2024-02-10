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
                if config.number_lines {
                    for line in buf.lines() {
                        println!("{:>6}\t{}", line_number, line?);
                        line_number += 1;
                    }
                } else if config.number_nonblank_lines {
                    for line in buf.lines() {
                        match line {
                            Ok(l) if l == "".to_string() => println!(""),
                            Ok(l) => {
                                println!("{:>6}\t{}", line_number, l);
                                line_number += 1;
                            }
                            Err(e) => eprintln!("Failed to read line: {}", e),
                        }
                    }
                } else {
                    for line in buf.lines() {
                        println!("{}", line?);
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
