use crate::utils::symbol::Symbol;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub symbols: HashMap<String, Symbol>,
    pub parent: Option<Box<SymbolTable>>,
}

impl SymbolTable {
    pub fn new(parent: Option<Box<SymbolTable>>) -> Self {
        Self {
            symbols: HashMap::new(),
            parent,
        }
    }

    pub fn get(&mut self, name: String) -> Option<&Symbol> {
        if self.symbols.contains_key(&name) {
            return self.symbols.get(&name);
        }

        if self.parent.clone().is_some() {
            return self.parent.as_ref().unwrap().symbols.get(&name);
        }

        None
    }

    pub fn set(mut self, name: String, val: Symbol) -> Self {
        self.symbols.insert(name, val);
        self
    }

    pub fn delete(mut self, name: String) {
        self.symbols.remove(&name);
    }
}
