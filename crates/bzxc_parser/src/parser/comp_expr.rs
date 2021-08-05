/*
 * Copyright 2020 to 2021 BlazifyOrg
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *    http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
*/

use super::Parser;
use crate::parse_result::ParseResult;
use bzxc_shared::{Error, Node, Tokens};

impl Parser {
    /*
     * Parse a computed expression
     */
    pub(crate) fn comp_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let pos_start = self.current_token.clone().pos_start;

        if self.current_token.value == Tokens::Keyword("not") {
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
        .contains(&self.current_token.value)
        {
            let op_token = self.current_token.clone();
            res.register_advancement();
            self.advance();

            let right = res.register(self.arith_expr());
            if res.error.is_some() {
                return res;
            }
            left = Option::from(Node::BinaryNode {
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
