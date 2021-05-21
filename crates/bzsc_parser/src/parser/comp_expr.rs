use super::Parser;
use crate::parse_result::ParseResult;
use bzs_shared::{DynType, Error, Node, Tokens};

impl Parser {
    /*
     * Parse a computed expression
     */
    pub(crate) fn comp_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let pos_start = self.current_token.clone().pos_start;

        if self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("not".to_string()))
        {
            let op_token = self.current_token.clone();
            res.register_advancement();
            self.advance();

            let node = res.register(self.comp_expr());
            if res.error.is_some() {
                return res;
            }

            return res.success(Node::UnaryNode {
                node: Box::new(node.clone().unwrap()),
                op_token: op_token.clone(),
            });
        }

        let mut left = res.register(self.arith_expr());
        if res.error.is_some() {
            return res;
        }

        while [
            Tokens::DoubleEquals,
            Tokens::NotEquals,
            Tokens::LessThan,
            Tokens::LessThanEquals,
            Tokens::GreaterThan,
            Tokens::GreaterThanEquals,
        ]
        .contains(&self.current_token.r#type)
        {
            let op_token = self.current_token.clone();
            res.register_advancement();
            self.advance();

            let right = res.register(self.arith_expr());
            if res.error.is_some() {
                return res;
            }
            left = Option::from(Node::BinOpNode {
                left: Box::new(left.clone().unwrap()),
                right: Box::new(right.clone().unwrap()),
                op_token,
            });
        }

        if res.error.is_some() {
            return res.failure(Error::new(
                "Invalid Syntax",
                pos_start,
                self.current_token.pos_end.clone(),
                "A Int or Float or Identifier, '+', '-', '(', 'not', '!' was Expected",
            ));
        }
        res.success(left.unwrap())
    }
}
