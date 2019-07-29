use itertools::Itertools;

use crate::error::LexingError;
use crate::error::LoxResult;
use crate::token::Token;
use crate::token::TokenType;
use crate::token::TokenType::*;

pub fn scan_tokens(text: &str) -> LoxResult<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();

    {
        let mut line_num: usize = 0;

        let mut letters = text
            .chars()
            .chain(vec!['\0'].into_iter())
            .tuple_windows()
            .enumerate();
        while let Some((idx, letter)) = letters.next() {
            let make_token = |tt: TokenType, slice_size: usize| {
                Token::new(tt, text[idx..idx + slice_size].to_string(), line_num as u32)
            };
            match letter {
                ('(', _) => tokens.push(make_token(LeftParen, 1)),
                (')', _) => tokens.push(make_token(RightParen, 1)),
                ('{', _) => tokens.push(make_token(LeftBrace, 1)),
                ('}', _) => tokens.push(make_token(RightBrace, 1)),
                (',', _) => tokens.push(make_token(Comma, 1)),
                ('.', _) => tokens.push(make_token(Dot, 1)),
                ('-', _) => tokens.push(make_token(Minus, 1)),
                ('+', _) => tokens.push(make_token(Plus, 1)),
                (';', _) => tokens.push(make_token(Semicolon, 1)),
                ('*', _) => tokens.push(make_token(Star, 1)),
                ('!', '=') => {
                    tokens.push(make_token(BangEqual, 2));
                    letters.next();
                }
                ('!', _) => tokens.push(make_token(Bang, 1)),
                ('=', '=') => {
                    tokens.push(make_token(EqualEqual, 2));
                    letters.next();
                }
                ('=', _) => tokens.push(make_token(Equal, 1)),
                ('<', '=') => {
                    tokens.push(make_token(LessEqual, 2));
                    letters.next();
                }
                ('<', _) => tokens.push(make_token(Less, 1)),
                ('>', '=') => {
                    tokens.push(make_token(GreaterEqual, 2));
                    letters.next();
                }
                ('>', _) => tokens.push(make_token(Greater, 1)),
                ('/', '/') => {
                    // Ignore rest of comment
                    loop {
                        if let Some((_, (_, '\n'))) = letters.next() {
                            break;
                        }
                    }
                }
                ('/', _) => tokens.push(make_token(Slash, 1)),
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
                                    String_(string_lit.iter().collect()),
                                    string_lit.len() + 2,
                                ));
                                break;
                            }
                            Some((_idx, (chr, _))) => {
                                string_lit.push(chr);
                            }
                            None => {
                                return Err(LexingError::UnexpectedEndStringLiteral { line_num })?
                            }
                        }
                    }
                }
                (first @ '0'...'9', second) => handle_number(
                    &mut tokens,
                    &mut letters,
                    make_token,
                    (idx, (first, second)),
                    line_num,
                )?,
                (lett @ 'a'...'z', _) | (lett @ 'A'...'Z', _) => {
                    handle_ident_or_keyword(&mut tokens, &mut letters, make_token, lett)?
                }
                (first, _) => return Err(LexingError::InvalidToken(first))?,
            }
        }
    }
    Ok(tokens)
}

#[inline]
fn handle_ident_or_keyword(
    tokens: &mut Vec<Token>,
    letters: &mut impl Iterator<Item = (usize, (char, char))>,
    make_token: impl Fn(TokenType, usize) -> Token,
    lett: char,
) -> LoxResult<()> {
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

#[inline]
fn make_ident_or_keyword(
    identifier_lit: &[char],
    make_token: impl Fn(TokenType, usize) -> Token,
) -> Token {
    use crate::token::RESERVED_TOKENS;

    let identifier: String = identifier_lit.iter().collect();
    if let Some(reserved) = RESERVED_TOKENS.get(identifier.as_str()) {
        make_token(reserved.clone(), identifier_lit.len())
    } else {
        make_token(Identifier(identifier), identifier_lit.len())
    }
}

#[inline]
fn handle_number(
    tokens: &mut Vec<Token>,
    letters: &mut impl Iterator<Item = (usize, (char, char))>,
    make_token: impl Fn(TokenType, usize) -> Token,
    lett: (usize, (char, char)),
    line_num: usize,
) -> LoxResult<()> {
    let mut num_lit: Vec<char> = vec![];
    let mut next_letter: Option<(usize, (char, char))> = Some(lett);
    loop {
        match next_letter {
            Some((_idx, (chr, next))) if !next.is_alphanumeric() && next != '.' => {
                num_lit.push(chr);
                tokens.push(make_number(&num_lit, make_token, line_num)?);
                break;
            }
            Some((_idx, (chr, _))) => {
                num_lit.push(chr);
            }
            None => {
                tokens.push(make_number(&num_lit, make_token, line_num)?);
                break;
            }
        }
        next_letter = letters.next()
    }
    Ok(())
}

#[inline]
fn make_number(
    num_lit: &[char],
    make_token: impl Fn(TokenType, usize) -> Token,
    line_num: usize,
) -> LoxResult<Token> {
    let num: String = num_lit.iter().collect();
    Ok(make_token(
        Number(
            num.parse()
                .map_err(|err| LexingError::InvalidDigit { line_num, err })?,
        ),
        num_lit.len(),
    ))
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::error::LoxError;

    use super::*;

    #[test]
    fn test_good_numbers() {
        ["var foo = 123;", "var foo = 123\n\n\n", "var foo = 123"]
            .iter()
            .for_each(|&text| {
                let res = scan_tokens(text).unwrap();
                assert_eq!(&res[3], &Token::new(Number(123f64), "123".into(), 0));
            })
    }

    #[test]
    fn test_invalid_number() {
        let example = r#"var foo = 123f456"#;
        let res = scan_tokens(example);
        match res.expect_err("Should have failed to parse invalid number") {
            LoxError::InnerLexingError(LexingError::InvalidDigit {
                line_num: _,
                err: _,
            }) => {}
            err => panic!("Wrong error type {:?}. Expected Invalid Digit.", err),
        }
    }

    #[test]
    fn test_string_double_eq_number() {
        let example = r#""asdf" == 123.456"#;
        let res = scan_tokens(example).unwrap();

        assert_eq!(
            &res[0],
            &Token::new(String_("asdf".into()), r#""asdf""#.into(), 0)
        );

        assert_eq!(&res[1], &Token::new(EqualEqual, "==".into(), 0));

        assert_eq!(&res[2], &Token::new(Number(123.456), "123.456".into(), 0));
    }

    #[test]
    fn test_line_number() {
        let example = "\n123.456";
        let res = scan_tokens(example).unwrap();

        assert_eq!(&res[0], &Token::new(Number(123.456), "123.456".into(), 1));
    }

    #[test]
    fn test_identifier() {
        let example = "var foobar = 123.456";
        let res = scan_tokens(example).unwrap();

        assert_eq!(&res[0], &Token::new(Var, "var".into(), 0));

        assert_eq!(
            &res[1],
            &Token::new(Identifier("foobar".into()), "foobar".into(), 0)
        );

        assert_eq!(&res[2], &Token::new(Equal, "=".into(), 0));

        assert_eq!(&res[3], &Token::new(Number(123.456), "123.456".into(), 0));
    }

    #[test]
    fn test_comments() {
        let example = "//HIII THEREEEEE\nvar";
        let res = scan_tokens(example).unwrap();

        assert_eq!(&res[0], &Token::new(Var, "var".into(), 1));
    }
}
