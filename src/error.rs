use std::io;

use crate::token::{Token, TokenType};
use failure::Fail;
use std::num::ParseFloatError;

pub type LoxResult<T> = Result<T, LoxError>;

#[derive(Debug, Fail)]
pub enum LoxError {
    #[fail(display = "IO Error: {}", _0)]
    IoError(#[cause] io::Error),
    #[fail(display = "Lexing Error: {}", _0)]
    InnerLexingError(#[cause] LexingError),
    #[fail(display = "Parsing Error: {}", _0)]
    InnerParsingError(#[cause] ParsingError),
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

#[derive(Debug, Fail)]
pub enum ParsingError {
    #[fail(display = "Unexpected Token '{:?}'", _0)]
    UnexpectedToken(Token),
    #[fail(display = "Expected Token '{:?}'", _0)]
    ExpectedToken(TokenType),
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

impl From<ParsingError> for LoxError {
    fn from(err: ParsingError) -> Self {
        LoxError::InnerParsingError(err)
    }
}
