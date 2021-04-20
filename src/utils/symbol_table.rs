use crate::core::interpreter::r#type::Type;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub symbols: HashMap<String, Type>,
    pub parent: Option<Box<SymbolTable>>,
}

impl SymbolTable {
    pub fn new(parent: Option<Box<SymbolTable>>) -> Self {
        Self {
            symbols: HashMap::new(),
            parent,
        }
    }

    pub fn get(&mut self, name: String) -> Option<&Type> {
        if self.symbols.contains_key(&name) {
            return self.symbols.get(&name);
        }

        return self.parent.as_ref().unwrap().symbols.get(&name);
    }

    pub fn set(mut self, name: String, val: Type) -> Type {
        self.symbols.insert(name, val).unwrap()
    }

    pub fn delete(mut self, name: String) {
        self.symbols.remove(&name);
    }
}
