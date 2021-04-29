use crate::utils::symbol::Symbol;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    pub display_name: String,
    pub symbols: HashMap<String, Symbol>,
}

impl Context {
    pub fn new(display_name: String) -> Self {
        Self {
            display_name,
            symbols: HashMap::new(),
        }
    }
}
