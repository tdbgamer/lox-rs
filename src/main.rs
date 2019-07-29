use std::fs::File;
use std::io::stdout;
use std::io::{self, BufRead, BufReader, Write};

use structopt::StructOpt;

use crate::ast_interpreter::AstIntepreter;
use crate::interpreter::LoxInterpreter;
use crate::parser::parse_tokens;
use error::LoxResult;
use lexer::scan_tokens;

mod ast;
mod ast_interpreter;
mod error;
mod interpreter;
mod lexer;
mod parser;
mod token;
mod types;

#[derive(StructOpt)]
struct Cli {
    #[structopt(help = "Input file")]
    input: Option<String>,
}

fn open_file(input_file: &str) -> LoxResult<Box<dyn BufRead>> {
    match input_file {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        filename => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn main() -> LoxResult<()> {
    let vm = AstIntepreter::default();
    let args: Cli = Cli::from_args();
    let input_file = args.input.unwrap_or_else(|| "-".into());
    let mut file = open_file(&input_file)?;
    loop {
        print!("> ");
        stdout().flush()?;
        let mut input_str = String::new();
        file.read_line(&mut input_str)?;
        println!("{:#?}", vm.eval(input_str.as_str())?);
    }
}
