use bzs_shared::{to_static_str, DynType, Error, Tokens, Type};

use crate::parse_result::ParseResult;

use super::Parser;

impl Parser {
    pub(crate) fn type_expr(&mut self, res: &mut ParseResult) -> Result<Type, Error> {
        let pos_start = self.current_token.pos_start.clone();

        if self.current_token.typee != Tokens::Colon {
            return Err(Error::new(
                "Invalid Syntax",
                pos_start,
                self.current_token.pos_end.clone(),
                "Expected ':'",
            ));
        }

        res.register_advancement();
        self.advance();

        if self.current_token.typee != Tokens::Identifier
            && self.current_token.typee != Tokens::Keyword
        {
            return Err(Error::new(
                "Invalid Syntax",
                pos_start,
                self.current_token.pos_end.clone(),
                "Expected Type",
            ));
        }

        match &self.current_token.value.clone() {
            DynType::String(typee) => {
                res.register_advancement();
                self.advance();

                match typee.as_str() {
                    "int" => Ok(Type::Int),
                    "float" => Ok(Type::Float),
                    "char" => Ok(Type::Char),
                    "boolean" => Ok(Type::Boolean),
                    "string" => Ok(Type::String),
                    "void" => Ok(Type::Void),
                    _ => Ok(Type::Custom(to_static_str(typee.clone()))),
                }
            }
            _ => Err(Error::new(
                "Invalid Syntax",
                pos_start,
                self.current_token.pos_end.clone(),
                "Expected Type",
            )),
        }
    }
}
