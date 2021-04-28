use crate::utils::{position::Position, symbol_table::SymbolTable};

#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    pub display_name: String,
    pub symbol_table: SymbolTable,
    pub parent: Box<Option<Context>>,
    pub parent_position: Option<Position>,
}

impl Context {
    pub fn new(
        display_name: String,
        symbol_table: SymbolTable,
        parent: Box<Option<Context>>,
        parent_position: Option<Position>,
    ) -> Self {
        Self {
            display_name,
            symbol_table,
            parent,
            parent_position,
        }
    }
}
