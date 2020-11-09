use crate::core::token::Token;
use crate::utils::position::Position;

#[derive(Debug, Clone)]
pub struct StringNode {
    pub token: Token,
    pub pos_start: Position,
    pub pos_end: Position,
}

impl StringNode {
    pub fn new(token: Token) -> StringNode {
        StringNode {
            token: token.clone(),
            pos_start: token.clone().pos_start.clone(),
            pos_end: token.clone().pos_end.clone(),
        }
    }
}
