use std::io;

use failure::Fail;
use std::num::ParseFloatError;

pub type LoxResult<T> = Result<T, LoxError>;

#[derive(Debug, Fail)]
pub enum LoxError {
    #[fail(display = "IO Error: {}", _0)]
    IoError(#[cause] io::Error),
    #[fail(display = "Lexing Error: {}", _0)]
    InnerLexingError(#[cause] LexingError),
}

#[derive(Debug, Fail)]
pub enum LexingError {
    #[fail(display = "Invalid Token '{}'", _0)]
    InvalidToken(char),
    #[fail(
        display = "String literal unexpected ended on line number {}",
        line_num
    )]
    UnexpectedEndStringLiteral { line_num: usize },
    #[fail(
        display = "Could not parse digit on line number {}. Failed with error: {}",
        line_num, err
    )]
    InvalidDigit {
        line_num: usize,
        #[cause]
        err: ParseFloatError,
    },
}

impl From<io::Error> for LoxError {
    fn from(err: io::Error) -> Self {
        LoxError::IoError(err)
    }
}

impl From<LexingError> for LoxError {
    fn from(err: LexingError) -> Self {
        LoxError::InnerLexingError(err)
    }
}
