use bzxc_shared::{Error, Node, Tokens};

use crate::parse_result::ParseResult;

use super::Parser;

impl Parser {
    /*
     * Variable related expressions
     */
    pub(crate) fn var_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        if let Tokens::Identifier(_) = self.current_token.value {
        } else {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected Identifier",
            ));
        }

        let tok = self.current_token.clone();
        self.advance();
        res.register_advancement();

        let type_tok = self.current_token.clone();
        if [
            Tokens::Equals,
            Tokens::PlusEquals,
            Tokens::MinusEquals,
            Tokens::MultiplyEquals,
            Tokens::DivideEquals,
            Tokens::PowerEquals,
            Tokens::ModuloEquals,
        ]
        .contains(&type_tok.value)
        {
            res.register_advancement();
            self.advance();

            let expr = res.register(self.expr());
            if res.error.is_some() {
                return res;
            }

            return res.success(Node::VarReassignNode {
                name: tok,
                typee: type_tok,
                value: Box::new(expr.unwrap()),
            });
        }

        res.success(Node::VarAccessNode { token: tok })
    }
}
