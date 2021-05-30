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
use bzc_shared::{DynType, Error, Node, Tokens};

impl Parser {
    /*
     * Parses a For loop
     */
    pub(crate) fn for_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        if !self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("for".to_string()))
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected 'for'",
            ));
        }

        res.register_advancement();
        self.advance();

        if self.current_token.typee != Tokens::Identifier {
            return res.failure(Error::new(
                "Invalid Syntax",
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

        let init_expr = res.register(self.expr());
        if res.error.is_some() {
            return res;
        }

        if !self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("to".to_string()))
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected 'to'",
            ));
        }

        res.register_advancement();
        self.advance();

        let end_expr = res.register(self.expr());
        if res.error.is_some() {
            return res;
        }

        if !self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("step".to_string()))
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start,
                self.current_token.pos_end,
                "Expected 'step' keyword",
            ));
        }

        res.register_advancement();
        self.advance();
        let expr = res.register(self.expr());
        if res.error.is_some() {
            return res;
        }
        let step = expr.unwrap();

        if !self
            .current_token
            .clone()
            .matches(Tokens::LeftCurlyBraces, DynType::None)
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '{'",
            ));
        }

        res.register_advancement();
        self.advance();

        let body = res.register(self.statements());
        if res.error.is_some() {
            return res;
        }

        if !self
            .current_token
            .clone()
            .matches(Tokens::RightCurlyBraces, DynType::None)
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '}'",
            ));
        }

        res.register_advancement();
        self.advance();

        res.success(Node::ForNode {
            var_name_token: var_name,
            start_value: Box::new(init_expr.clone().unwrap()),
            end_value: Box::new(end_expr.clone().unwrap()),
            body_node: Box::new(body.clone().unwrap()),
            step_value_node: Box::new(step.clone()),
        })
    }
}
