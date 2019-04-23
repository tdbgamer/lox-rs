use std::io;

use failure::Fail;
use std::num::ParseFloatError;

pub type TimResult<T> = Result<T, TimError>;

#[derive(Debug, Fail)]
pub enum TimError {
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
    #[fail(display = "Could not parse identifier on line number {}", line_num)]
    InvalidIdentifier { line_num: usize },
}

impl From<io::Error> for TimError {
    fn from(err: io::Error) -> Self {
        TimError::IoError(err)
    }
}

impl From<LexingError> for TimError {
    fn from(err: LexingError) -> Self {
        TimError::InnerLexingError(err)
    }
}
