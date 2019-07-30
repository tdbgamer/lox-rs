use std::collections::HashMap;
use crate::types::LoxType;

#[derive(Default)]
pub struct Frame {
    locals: HashMap<String, LoxType>
}
