use std;
use std::io::Read;
use std::iter::{Chain, Enumerate};
use std::str::Chars;
use std::vec::IntoIter;

use itertools::structs::TupleWindows;
use itertools::Itertools;

use crate::error::LexingError;
use crate::error::TimResult;
use crate::token::Token;
use crate::token::TokenType;
use crate::token::TokenType::*;
use crate::types::LoxType;

#[derive(Default)]
pub struct Lexer {}

impl Lexer {
    pub fn scan_tokens(&self, mut input: Box<Read>) -> TimResult<Vec<Token>> {
        let mut tokens: Vec<Token> = Vec::new();

        {
            let mut text = String::new();
            input.read_to_string(&mut text)?;

            let mut line_num: usize = 0;

            let mut letters = text
                .chars()
                .chain(vec!['\0'].into_iter())
                .tuple_windows()
                .enumerate();
            while let Some((idx, letter)) = letters.next() {
                let make_token = |tt: TokenType, slice_size: usize, literal: Option<LoxType>| {
                    Token::new(
                        tt,
                        text[idx..idx + slice_size].to_string(),
                        literal,
                        line_num as u32,
                    )
                };
                match letter {
                    ('(', _) => tokens.push(make_token(LeftParen, 1, None)),
                    (')', _) => tokens.push(make_token(RightParen, 1, None)),
                    ('{', _) => tokens.push(make_token(LeftBrace, 1, None)),
                    ('}', _) => tokens.push(make_token(RightBrace, 1, None)),
                    (',', _) => tokens.push(make_token(Comma, 1, None)),
                    ('.', _) => tokens.push(make_token(Dot, 1, None)),
                    ('-', _) => tokens.push(make_token(Minus, 1, None)),
                    ('+', _) => tokens.push(make_token(Plus, 1, None)),
                    (';', _) => tokens.push(make_token(Semicolon, 1, None)),
                    ('*', _) => tokens.push(make_token(Star, 1, None)),
                    ('!', '=') => {
                        tokens.push(make_token(BangEqual, 2, None));
                        letters.next();
                    }
                    ('!', _) => tokens.push(make_token(Bang, 1, None)),
                    ('=', '=') => {
                        tokens.push(make_token(EqualEqual, 2, None));
                        letters.next();
                    }
                    ('=', _) => tokens.push(make_token(Equal, 1, None)),
                    ('<', '=') => {
                        tokens.push(make_token(LessEqual, 2, None));
                        letters.next();
                    }
                    ('<', _) => tokens.push(make_token(Less, 1, None)),
                    ('>', '=') => {
                        tokens.push(make_token(GreaterEqual, 2, None));
                        letters.next();
                    }
                    ('>', _) => tokens.push(make_token(Greater, 1, None)),
                    ('/', '/') => {
                        // Ignore rest of comment
                        loop {
                            match letters.next() {
                                Some((_, ('\n', _))) => break,
                                _ => {}
                            }
                        }
                    }
                    ('/', _) => tokens.push(make_token(Slash, 1, None)),
                    (' ', _) => {}
                    ('\r', _) => {}
                    ('\t', _) => {}
                    ('\n', _) => {
                        line_num += 1;
                    }
                    ('"', _) => {
                        let mut string_lit: Vec<char> = Vec::new();
                        loop {
                            let next_letter = letters.next();
                            match next_letter {
                                Some((idx, ('"', _))) => {
                                    tokens.push(make_token(
                                        String_,
                                        string_lit.len() + 2,
                                        Some(LoxType::String_(string_lit.iter().collect())),
                                    ));
                                    break;
                                }
                                Some((idx, (chr, _))) => {
                                    string_lit.push(chr);
                                }
                                None => {
                                    return Err(LexingError::UnexpectedEndStringLiteral {
                                        line_num,
                                    })?
                                }
                            }
                        }
                    }
                    (first @ '0'...'9', _) => {
                        let mut num_lit: Vec<char> = vec![first];
                        loop {
                            let next_letter: Option<(usize, (char, char))> = letters.next();
                            match next_letter {
                                Some((idx, (chr, next)))
                                    if !(next.is_ascii_digit() || next == '.') =>
                                {
                                    num_lit.push(chr);
                                    let num: String = num_lit.iter().collect();
                                    tokens.push(make_token(
                                        Number,
                                        num_lit.len(),
                                        Some(LoxType::Number(num.parse().map_err(|err| {
                                            LexingError::InvalidDigit { line_num, err }
                                        })?)),
                                    ));
                                    break;
                                }
                                Some((idx, (chr, _))) => {
                                    num_lit.push(chr);
                                }
                                None => {
                                    let num: String = num_lit.iter().collect();
                                    tokens.push(make_token(
                                        Number,
                                        num_lit.len(),
                                        Some(LoxType::Number(num.parse().map_err(|err| {
                                            LexingError::InvalidDigit { line_num, err }
                                        })?)),
                                    ));
                                    break;
                                }
                            }
                        }
                    }
                    (first, _) => return Err(LexingError::InvalidToken(first))?,
                }
            }
        }
        Ok(tokens)
    }
}
