use crate::types::LoxType;

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(LoxType),
    Unary(UnaryOp, Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Bang,
    Minus,
}

#[derive(Debug, Clone)]
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
