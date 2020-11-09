use crate::core::token::Token;
use crate::utils::position::Position;

#[derive(Debug, Clone)]
pub struct NumberNode {
    pub token: Token,
    pub pos_start: Position,
    pub pos_end: Position,
}

impl NumberNode {
    pub fn new(token: Token) -> NumberNode {
        NumberNode {
            token: token.clone(),
            pos_start: token.clone().pos_start.clone(),
            pos_end: token.clone().pos_end.clone(),
        }
    }
}
