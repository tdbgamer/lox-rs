use std;
use std::io::Read;

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
                                Some((_, (_, '\n'))) => break,
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
                                Some((_idx, ('"', _))) => {
                                    tokens.push(make_token(
                                        String_,
                                        string_lit.len() + 2,
                                        Some(LoxType::String_(string_lit.iter().collect())),
                                    ));
                                    break;
                                }
                                Some((_idx, (chr, _))) => {
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
                                Some((_idx, (chr, next)))
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
                                Some((_idx, (chr, _))) => {
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
                    (lett @ 'a'...'z', _) | (lett @ 'A'...'Z', _) => {
                        handle_ident_or_keyword(&mut tokens, &mut letters, make_token, lett)?
                    }
                    (first, _) => return Err(LexingError::InvalidToken(first))?,
                }
            }
        }
        Ok(tokens)
    }
}

fn handle_ident_or_keyword(
    tokens: &mut Vec<Token>,
    letters: &mut impl Iterator<Item = (usize, (char, char))>,
    make_token: impl Fn(TokenType, usize, Option<LoxType>) -> Token,
    lett: char,
) -> TimResult<()> {
    let mut identifier_lit = vec![lett];
    loop {
        let next_letter: Option<(usize, (char, char))> = letters.next();
        match next_letter {
            Some((_, (first, _))) => match first {
                'a'...'z' | 'A'...'Z' | '0'...'9' => identifier_lit.push(first),
                _ => {
                    tokens.push(make_ident_or_keyword(&identifier_lit, &make_token));
                    break;
                }
            },
            None => {
                tokens.push(make_ident_or_keyword(&identifier_lit, &make_token));
                break;
            }
        };
    }
    Ok(())
}

fn make_ident_or_keyword(
    identifier_lit: &[char],
    make_token: impl Fn(TokenType, usize, Option<LoxType>) -> Token,
) -> Token {
    use crate::token::RESERVED_TOKENS;

    let identifier: String = identifier_lit.iter().collect();
    if let Some(reserved) = RESERVED_TOKENS.get(identifier.as_str()) {
        make_token(reserved.clone(), identifier_lit.len(), None)
    } else {
        make_token(
            Identifier,
            identifier_lit.len(),
            Some(LoxType::Identifier(identifier)),
        )
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_string_double_eq_number() {
        let example = r#""asdf" == 123.456"#;
        let cur = Cursor::new(example);
        let res = Lexer::default().scan_tokens(Box::new(cur)).unwrap();

        assert_eq!(
            &res[0],
            &Token::new(
                String_,
                r#""asdf""#.into(),
                Some(LoxType::String_("asdf".into())),
                0
            )
        );

        assert_eq!(&res[1], &Token::new(EqualEqual, "==".into(), None, 0));

        assert_eq!(
            &res[2],
            &Token::new(Number, "123.456".into(), Some(LoxType::Number(123.456)), 0)
        );
    }

    #[test]
    fn test_line_number() {
        let example = "\n123.456";
        let cur = Cursor::new(example);
        let res = Lexer::default().scan_tokens(Box::new(cur)).unwrap();

        assert_eq!(
            &res[0],
            &Token::new(Number, "123.456".into(), Some(LoxType::Number(123.456)), 1)
        );
    }

    #[test]
    fn test_identifier() {
        let example = "var foobar = 123.456";
        let cur = Cursor::new(example);
        let res = Lexer::default().scan_tokens(Box::new(cur)).unwrap();

        assert_eq!(&res[0], &Token::new(Var, "var".into(), None, 0));

        assert_eq!(
            &res[1],
            &Token::new(
                Identifier,
                "foobar".into(),
                Some(LoxType::Identifier("foobar".into())),
                0
            )
        );

        assert_eq!(&res[2], &Token::new(Equal, "=".into(), None, 0));

        assert_eq!(
            &res[3],
            &Token::new(Number, "123.456".into(), Some(LoxType::Number(123.456)), 0)
        );
    }

    #[test]
    fn test_comments() {
        let example = "//HIII THEREEEEE\nvar";
        let cur = Cursor::new(example);
        let res = Lexer::default().scan_tokens(Box::new(cur)).unwrap();

        assert_eq!(&res[0], &Token::new(Var, "var".into(), None, 1));
    }
}
