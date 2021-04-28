use crate::utils::{constants::DynType, constants::Tokens, position::Position};
use std::fmt::{Display, Error, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub r#type: Tokens,
    pub value: DynType,
    pub pos_start: Position,
    pub pos_end: Position,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        return if self.value == DynType::None {
            write!(f, "[{:?}]", self.r#type)
        } else {
            write!(f, "[{:?}]", self.value)
        };
    }
}

impl Token {
    pub fn new(r#type: Tokens, pos_start: Position, pos_end: Position, value: DynType) -> Token {
        Token {
            r#type,
            value,
            pos_start,
            pos_end,
        }
    }

    pub fn clone(&mut self) -> Token {
        Token {
            r#type: *&self.r#type,
            value: self.value.clone(),
            pos_start: self.pos_start.clone(),
            pos_end: self.pos_end.clone(),
        }
    }

    pub fn matches(self, r#type: Tokens, value: DynType) -> bool {
        return self.r#type == r#type && self.value == value;
    }
}
