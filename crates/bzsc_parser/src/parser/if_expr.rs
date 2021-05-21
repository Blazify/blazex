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
     * Parses a If Expresion
     */
    pub(crate) fn if_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        if !self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("if".to_string()))
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected 'if'",
            ));
        }

        res.register_advancement();
        self.advance();

        let mut cases: Vec<(Node, Node)> = vec![];
        let mut else_case: Option<Node> = None;

        let first_condition = res.register(self.expr());
        if res.error.is_some() {
            return res;
        }

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

        let first_expr = res.register(self.statements());
        if res.error.is_some() {
            return res;
        }

        cases.push((first_condition.unwrap(), first_expr.unwrap()));

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
        self.advance();
        res.register_advancement();

        while self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("else".to_string()))
        {
            res.register_advancement();
            self.advance();

            if self
                .current_token
                .clone()
                .matches(Tokens::Keyword, DynType::String("if".to_string()))
            {
                res.register_advancement();
                self.advance();

                let condition = res.register(self.expr());
                if res.error.is_some() {
                    return res;
                }

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

                let else_if = res.register(self.statements());
                if res.error.is_some() {
                    return res;
                }

                cases.push((condition.unwrap(), else_if.unwrap()));

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
            } else {
                if !self
                    .current_token
                    .clone()
                    .matches(Tokens::LeftCurlyBraces, DynType::None)
                {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected '}'",
                    ));
                }
                self.advance();
                res.register_advancement();

                let else_ = res.register(self.statements());
                if res.error.is_some() {
                    return res;
                }

                else_case = Some(else_.unwrap());
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
                break;
            }
        }
        res.success(Node::IfNode {
            cases,
            else_case: Box::new(else_case.clone()),
        })
    }
}
