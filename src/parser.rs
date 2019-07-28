use crate::ast::{BinaryOp, Expr, UnaryOp};
use crate::error::{LoxResult, ParsingError};
use crate::token::{Token, TokenType};
use crate::types::LoxType;

macro_rules! binary_rule {
            ($rule:ident, $next_priority:ident, $( $token_type:ident )*) => {
                fn $rule(tokens: &[Token]) -> LoxResult<(Expr, &[Token])> {
                    let (expr, tail) = $next_priority(tokens)?;
                    match &tail.first() {
                        $(
                            Some(Token { token_type: TokenType::$token_type, .. }) => {
                                let (inner_expr, inner_tail) = equality(&tail[1..])?;
                                Ok((Expr::Binary(
                                    Box::new(expr),
                                    BinaryOp::$token_type,
                                    Box::new(inner_expr),
                                ), inner_tail))
                            }
                        )*
                        _ => { Ok((expr, tail)) }
                    }
                }
            }
        }

#[derive(Default)]
pub struct Parser;

impl Parser {
    pub fn parse_tokens(&self, tokens: &[Token]) -> LoxResult<Expr> {
        expression(tokens).map(|(expr, _)| expr)
    }
}

fn expression(tokens: &[Token]) -> LoxResult<(Expr, &[Token])> {
    equality(tokens)
}

binary_rule!(equality, comparison, EqualEqual BangEqual);
binary_rule!(comparison, addition, Greater GreaterEqual Less LessEqual);
binary_rule!(addition, multiplication, Plus Minus);
binary_rule!(multiplication, unary, Slash Star);

fn unary(tokens: &[Token]) -> LoxResult<(Expr, &[Token])> {
    let token = tokens.first().expect("First token didn't exist");
    match &token.token_type {
        TokenType::Bang => {
            let (expr, tail) = unary(&tokens[1..])?;
            Ok((Expr::Unary(UnaryOp::Bang, Box::new(expr)), tail))
        }
        TokenType::Minus => {
            let (expr, tail) = unary(&tokens[1..])?;
            Ok((Expr::Unary(UnaryOp::Minus, Box::new(expr)), tail))
        }
        _ => primary(tokens),
    }
}

fn primary(tokens: &[Token]) -> LoxResult<(Expr, &[Token])> {
    let token = tokens.first().expect("First token didn't exist");
    match &token.token_type {
        TokenType::Identifier(str_val) => Ok((
            Expr::Literal(LoxType::String_(str_val.clone())),
            &tokens[1..],
        )),
        TokenType::Number(num) => Ok((Expr::Literal(LoxType::Number(*num)), &tokens[1..])),
        TokenType::String_(str_val) => Ok((
            Expr::Literal(LoxType::String_(str_val.clone())),
            &tokens[1..],
        )),
        TokenType::False => Ok((Expr::Literal(LoxType::Boolean(false)), &tokens[1..])),
        TokenType::True => Ok((Expr::Literal(LoxType::Boolean(true)), &tokens[1..])),
        TokenType::Nil => Ok((Expr::Literal(LoxType::Nil), &tokens[1..])),
        TokenType::LeftParen => {
            let (expr, tail) = expression(&tokens[1..])?;
            match tail
                .first()
                .ok_or_else(|| ParsingError::ExpectedToken(TokenType::RightParen))?
            {
                Token {
                    token_type: TokenType::RightParen,
                    ..
                } => Ok((expr, &tail[1..])),
                token => Err(ParsingError::UnexpectedToken(token.clone()))?,
            }
        }
        _ => Err(ParsingError::UnexpectedToken(token.clone()))?,
    }
}
