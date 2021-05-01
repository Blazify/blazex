use crate::core::interpreter::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub value: Value,
    pub reassignable: bool,
}

impl Symbol {
    pub fn new(value: &Value, reassignable: bool) -> Self {
        Self {
            value: (*value).clone(),
            reassignable,
        }
    }
}
