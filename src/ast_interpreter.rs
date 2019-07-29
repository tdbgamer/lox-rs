use crate::ast::{BinaryOp, Expr, UnaryOp};
use crate::types::LoxType;

pub struct AstIntepreter;

impl AstIntepreter {
    pub fn eval(&self, ast: &Expr) -> Expr {
        match ast {
            lit @ Expr::Literal(_) => lit.clone(),
            Expr::Unary(op, expr) => {
                let val = self.eval(&**expr);
                match op {
                    UnaryOp::Bang => match val {
                        Expr::Literal(LoxType::Boolean(b)) => Expr::Literal(LoxType::Boolean(!b)),
                        _ => panic!("Unary operator '!' can only be used on booleans"),
                    },
                    UnaryOp::Minus => match val {
                        Expr::Literal(LoxType::Number(num)) => Expr::Literal(LoxType::Number(-num)),
                        _ => panic!("Unary operator '!' can only be used on booleans"),
                    },
                }
            }
            Expr::Binary(first, op, second) => {
                let first_val = &self.eval(first);
                let second_val = &self.eval(second);
                match op {
                    BinaryOp::BangEqual => self.eval(&Expr::Unary(
                        UnaryOp::Bang,
                        Box::new(Expr::Binary(
                            Box::new(*first.clone()),
                            BinaryOp::EqualEqual,
                            Box::new(*second.clone()),
                        )),
                    )),
                    BinaryOp::EqualEqual => match (first_val, second_val) {
                        (Expr::Literal(first_lit), Expr::Literal(second_lit)) => {
                            match (first_lit, second_lit) {
                                (LoxType::Number(first), LoxType::Number(second)) => {
                                    unimplemented!("Comparing floats")
                                }
                                (LoxType::Boolean(first), LoxType::Boolean(second)) => {
                                    Expr::Literal(LoxType::Boolean(first == second))
                                }
                                (LoxType::String_(first), LoxType::String_(second)) => {
                                    Expr::Literal(LoxType::Boolean(first == second))
                                }
                                (LoxType::Identifier(first), LoxType::Identifier(second)) => {
                                    unimplemented!("Comparing identifiers")
                                }
                                (LoxType::Identifier(first), second) => {
                                    unimplemented!("Comparing identifiers")
                                }
                                (first, LoxType::Identifier(second)) => {
                                    unimplemented!("Comparing identifiers")
                                }
                                (LoxType::Nil, LoxType::Nil) => {
                                    Expr::Literal(LoxType::Boolean(true))
                                }
                                (_, _) => Expr::Literal(LoxType::Boolean(false)),
                            }
                        }
                        _ => panic!(
                            "{:?} or {:?} did not evaluate to literals",
                            first_val, second_val
                        ),
                    },
                    BinaryOp::Greater => match (first_val, second_val) {
                        (Expr::Literal(LoxType::Number(first)), Expr::Literal(LoxType::Number(second))) =>
                            Expr::Literal(LoxType::Boolean(first > second)),
                        _ => panic!("Can't apply > on {:?} and {:?}", first_val, second_val),
                    },
                    BinaryOp::GreaterEqual => match (first_val, second_val) {
                        (Expr::Literal(LoxType::Number(first)), Expr::Literal(LoxType::Number(second))) =>
                            Expr::Literal(LoxType::Boolean(first >= second)),
                        _ => panic!("Can't apply >= on {:?} and {:?}", first_val, second_val),
                    },
                    BinaryOp::Less => match (first_val, second_val) {
                        (Expr::Literal(LoxType::Number(first)), Expr::Literal(LoxType::Number(second))) =>
                            Expr::Literal(LoxType::Boolean(first < second)),
                        _ => panic!("Can't apply < on {:?} and {:?}", first_val, second_val),
                    },
                    BinaryOp::LessEqual => match (first_val, second_val) {
                        (Expr::Literal(LoxType::Number(first)), Expr::Literal(LoxType::Number(second))) =>
                            Expr::Literal(LoxType::Boolean(first <= second)),
                        _ => panic!("Can't apply <= on {:?} and {:?}", first_val, second_val),
                    },
                    BinaryOp::Plus => match (first_val, second_val) {
                        (Expr::Literal(LoxType::Number(first)), Expr::Literal(LoxType::Number(second))) =>
                            Expr::Literal(LoxType::Number(first + second)),
                        _ => panic!("Can't add {:?} and {:?}", first_val, second_val),
                    },
                    BinaryOp::Minus => match (first_val, second_val) {
                        (Expr::Literal(LoxType::Number(first)), Expr::Literal(LoxType::Number(second))) =>
                            Expr::Literal(LoxType::Number(first - second)),
                        _ => panic!("Can't subtract {:?} and {:?}", first_val, second_val),
                    },
                    BinaryOp::Star => match (first_val, second_val) {
                        (Expr::Literal(LoxType::Number(first)), Expr::Literal(LoxType::Number(second))) =>
                            Expr::Literal(LoxType::Number(first * second)),
                        _ => panic!("Can't multiply {:?} and {:?}", first_val, second_val),
                    },
                    BinaryOp::Slash => match (first_val, second_val) {
                        (Expr::Literal(LoxType::Number(first)), Expr::Literal(LoxType::Number(second))) =>
                            Expr::Literal(LoxType::Number(first / second)),
                        _ => panic!("Can't divide {:?} and {:?}", first_val, second_val),
                    },
                }
            }
        }
    }
}
