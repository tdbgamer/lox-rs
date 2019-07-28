use std::fs::File;
use std::io::{self, BufReader, Read};

use structopt::StructOpt;

use error::LoxResult;
use lexer::Lexer;
use crate::parser::Parser;

mod ast;
mod error;
mod lexer;
mod parser;
mod token;
mod types;

#[derive(StructOpt)]
struct Cli {
    #[structopt(help = "Input file")]
    input: Option<String>,
}

fn open_file(input_file: &str) -> LoxResult<Box<dyn Read>> {
    match input_file {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        filename => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn main() -> LoxResult<()> {
    let args: Cli = Cli::from_args();
    let input_file = args.input.unwrap_or("-".into());
    let file = open_file(&input_file)?;
    let lexer: Lexer = Lexer::default();
    let tokens = lexer.scan_tokens(file)?;
    let parser = Parser;
    let tree = parser.parse_tokens(&tokens)?;
    println!("{:#?}", tree);
    Ok(())
}
