use crate::utils::symbol::Symbol;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
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

    pub fn get_exact(&self, name: String) -> Option<&Symbol> {
        self.symbols.get(&name)
    }

    pub fn get(&self, name: String) -> Option<&Symbol> {
        if self.symbols.contains_key(&name) {
            return self.symbols.get(&name);
        }

        if self.parent.clone().is_some() {
            return self.parent.as_ref().unwrap().get(name);
        }

        None
    }

    pub fn get_and_set(&mut self, name: String, new_symbol: Symbol) {
        if self.symbols.contains_key(&name) {
            self.symbols.insert(name.clone(), new_symbol);
        } else if let Some(parent) = &mut self.parent {
            parent.get_and_set(name, new_symbol);
        }
    }

    pub fn set(&mut self, name: String, val: Symbol) -> Self {
        self.symbols.insert(name, val);
        self.clone()
    }

    pub fn delete(&mut self, name: String) {
        self.symbols.remove(&name);
    }
}
