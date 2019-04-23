#[derive(Debug)]
pub enum LoxType {
    String_(String),
    Number(f64),
    Identifier(String),
}
