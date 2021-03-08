use crate::utils::{constants::DynType, constants::Tokens, position::Position};

#[derive(Debug, Clone)]
pub struct Token {
    pub r#type: Tokens,
    pub value: DynType,
    pub pos_start: Position,
    pub pos_end: Position,
}

impl Token {
    pub fn new(
        r#type: Tokens,
        pos_start: Position,
        pos_end: Position,
        value: Option<DynType>,
    ) -> Token {
        Token {
            r#type,
            value: value.unwrap_or(DynType::Int(0)),
            pos_start,
            pos_end,
        }
    }

    pub fn clone(&mut self) -> Token {
        let r#type = &self.r#type;
        Token {
            r#type: *r#type,
            value: self.value.clone(),
            pos_start: self.pos_start.clone(),
            pos_end: self.pos_end.clone(),
        }
    }

    pub fn matches(self, r#type: Tokens, value: DynType) -> bool {
        return self.r#type == r#type && self.value == value;
    }
}
