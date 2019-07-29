use crate::ast::Expr;
use crate::error::LoxResult;

pub trait LoxInterpreter<T> {
    fn eval(&self, input: T) -> LoxResult<Expr>;
}
