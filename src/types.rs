#[derive(Debug, PartialEq, Clone)]
pub enum LoxType {
    String_(String),
    Number(f64),
    Identifier(String),
    Boolean(bool),
    Nil,
}
