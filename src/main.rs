use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

use structopt::StructOpt;

use crate::parser::Parser;
use crate::ast_interpreter::AstIntepreter;
use error::LoxResult;
use lexer::Lexer;

mod ast;
mod error;
mod lexer;
mod parser;
mod token;
mod ast_interpreter;
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

use std::io::stdout;
fn main() -> LoxResult<()> {
    let vm = AstIntepreter;
    let args: Cli = Cli::from_args();
    let input_file = args.input.unwrap_or_else(|| "-".into());
    let mut file = open_file(&input_file)?;
    loop {
        print!("> ");
        stdout().flush()?;
        let mut input_str = String::new();
        file.read_line(&mut input_str)?;
        let lexer: Lexer = Lexer::default();
        let tokens = lexer.scan_tokens(&input_str)?;
        let parser = Parser;
        let tree = parser.parse_tokens(&tokens)?;
        println!("{:#?}", vm.eval(&tree));
    }
}
