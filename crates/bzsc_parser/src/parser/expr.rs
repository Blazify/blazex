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
use bzs_shared::{DynType, Error, Node, Tokens};

impl Parser {
    /*
     * Parse a expression
     */
    pub(crate) fn expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let pos_start = self.current_token.clone().pos_start;

        if self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String(String::from("val")))
            || self
                .current_token
                .clone()
                .matches(Tokens::Keyword, DynType::String(String::from("var")))
        {
            let var_type: String;
            match self.current_token.value.clone() {
                DynType::String(value) => var_type = value,
                _ => panic!(),
            };
            res.register_advancement();
            self.advance();

            if self.current_token.typee != Tokens::Identifier {
                return res.failure(Error::new(
                    "Invalid Syntax Error",
                    self.current_token.pos_start.clone(),
                    self.current_token.pos_end.clone(),
                    "Expected Identifier",
                ));
            }

            let var_name = self.current_token.clone();
            res.register_advancement();
            self.advance();

            if self.current_token.typee != Tokens::Equals {
                return res.failure(Error::new(
                    "Invalid Syntax",
                    self.current_token.pos_start.clone(),
                    self.current_token.pos_end.clone(),
                    "Expected '='",
                ));
            }

            res.register_advancement();
            self.advance();

            let expr = res.register(self.expr());
            if res.error.is_some() {
                return res;
            }

            let reassignable = if var_type == String::from("var") {
                true
            } else {
                false
            };
            return res.success(Node::VarAssignNode {
                name: var_name.clone(),
                value: Box::new(expr.unwrap()),
                reassignable,
            });
        }

        let mut left = res.register(self.comp_expr());
        if res.error.is_some() {
            return res;
        }

        while self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("and".to_string()))
            || self
                .current_token
                .clone()
                .matches(Tokens::Keyword, DynType::String("or".to_string()))
        {
            let op_token = self.current_token.clone();
            res.register_advancement();
            self.advance();
            let right = res.register(self.comp_expr());
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
                "Expected 'var', int, float, identifier, '+', '-' or '('",
            ));
        }

        res.success(left.unwrap())
    }
}
