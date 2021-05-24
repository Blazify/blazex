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
     * Parses a class initialization expression
     */
    pub(crate) fn class_init(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let mut constructor_params: Vec<Node> = vec![];

        if !self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("new".to_string()))
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected 'new'",
            ));
        }

        res.register_advancement();
        self.advance();

        if self.current_token.typee != Tokens::Identifier {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected identifier",
            ));
        }

        let name = self.current_token.clone();

        res.register_advancement();
        self.advance();

        if self.current_token.typee == Tokens::LeftParenthesis {
            res.register_advancement();
            self.advance();

            if self.current_token.typee == Tokens::RightParenthesis {
                res.register_advancement();
                self.advance();
            } else {
                let expr = res.register(self.expr());
                if res.error.is_some() {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected ')', 'var', int, float, identifier, '+', '-' or ','",
                    ));
                }
                constructor_params.push(expr.unwrap());

                while self.current_token.typee == Tokens::Comma {
                    res.register_advancement();
                    self.advance();

                    let expr = res.register(self.expr());
                    if res.error.is_some() {
                        return res.failure(Error::new(
                            "Invalid Syntax",
                            self.current_token.pos_start.clone(),
                            self.current_token.pos_end.clone(),
                            "Expected ')', 'var', int, float, identifier, '+', '-' or ','",
                        ));
                    }
                    constructor_params.push(expr.unwrap());
                }

                if self.current_token.typee != Tokens::RightParenthesis {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected ')' or ','",
                    ));
                }
                res.register_advancement();
                self.advance();
            }
        } else {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '('",
            ));
        }

        res.success(Node::ClassInitNode {
            name,
            constructor_params,
        })
    }
}
