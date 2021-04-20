use crate::core::interpreter::r#type::Type;

pub struct Symbol {
    pub value: Type,
    pub reassignable: bool,
}

impl Symbol {
    pub fn new(value: Type, reassignable: bool) -> Self {
        Self {
            value,
            reassignable,
        }
    }
}
