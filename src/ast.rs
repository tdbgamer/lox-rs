use crate::types::LoxType;

#[derive(Debug)]
pub enum Expr {
    Literal(LoxType),
    Unary(UnaryOp, Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Grouping(Box<Expr>),
}

#[derive(Debug)]
pub enum UnaryOp {
    Bang,
    Minus,
}

#[derive(Debug)]
pub enum BinaryOp {
    BangEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Plus,
    Minus,
    Star,
    Slash,
}
