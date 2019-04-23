use std::fs::File;
use std::io::{self, BufReader, Read};

use structopt::StructOpt;

use error::TimResult;
use lexer::Lexer;

mod error;
mod lexer;
mod token;
mod types;

#[derive(StructOpt)]
struct Cli {
    #[structopt(help = "Input file")]
    input: Option<String>,
}

fn open_file(input_file: &str) -> TimResult<Box<Read>> {
    match input_file {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        filename => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn main() -> TimResult<()> {
    let args: Cli = Cli::from_args();
    let input_file = args.input.unwrap_or("-".into());
    let file = open_file(&input_file)?;
    let lexer: Lexer = Lexer::default();
    let tokens = lexer.scan_tokens(file);
    println!("{:?}", tokens?);
    Ok(())
}
