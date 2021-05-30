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
use bzc_shared::{Error, Node, Token, Tokens};

impl Parser {
    /*
     * Parses a object expression
     */
    pub(crate) fn obj_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let pos_start = self.current_token.clone().pos_start;
        let mut properties: Vec<(Token, Node)> = vec![];

        if self.current_token.typee != Tokens::LeftCurlyBraces {
            return res.failure(Error::new(
                "Invalid syntax",
                pos_start,
                self.current_token.clone().pos_end,
                "'{' was expected.",
            ));
        }

        self.advance();
        res.register_advancement();

        if self.current_token.typee == Tokens::Newline {
            res.register_advancement();
            self.advance();
        }

        if self.current_token.typee == Tokens::RightCurlyBraces {
            res.register_advancement();
            self.advance();
        } else {
            let mut expr = res.register(self.expr());
            if res.error.is_some() {
                return res.failure(Error::new(
                    "Invalid syntax",
                    pos_start,
                    self.current_token.pos_end,
                    "'}', 'key' was expected.",
                ));
            }

            let mut tok;
            if let Node::StringNode { token, .. } = expr.unwrap() {
                tok = token;
            } else {
                return res.failure(Error::new(
                    "Invalid syntax",
                    pos_start,
                    self.current_token.clone().pos_end,
                    "string was expected.",
                ));
            }

            if self.current_token.typee != Tokens::Colon {
                return res.failure(Error::new(
                    "Invalid syntax",
                    pos_start,
                    self.current_token.clone().pos_end,
                    "':' was expected.",
                ));
            }

            res.register_advancement();
            self.advance();

            expr = res.register(self.expr());
            if res.error.is_some() {
                return res;
            }

            properties.push((tok, expr.unwrap()));

            while self.current_token.typee == Tokens::Comma {
                self.advance();
                res.register_advancement();

                if self.current_token.typee == Tokens::Newline {
                    res.register_advancement();
                    self.advance();
                }

                expr = res.register(self.expr());
                if res.error.is_some() {
                    return res.failure(Error::new(
                        "Invalid syntax",
                        pos_start,
                        self.current_token.pos_end,
                        "'}' ',', 'key' was expected.",
                    ));
                }

                if let Node::StringNode { token, .. } = expr.unwrap() {
                    tok = token;
                } else {
                    return res.failure(Error::new(
                        "Invalid syntax",
                        pos_start,
                        self.current_token.clone().pos_end,
                        "string was expected.",
                    ));
                }

                if self.current_token.typee != Tokens::Colon {
                    return res.failure(Error::new(
                        "Invalid syntax",
                        pos_start,
                        self.current_token.clone().pos_end,
                        "':' was expected.",
                    ));
                }

                res.register_advancement();
                self.advance();

                expr = res.register(self.expr());
                if res.error.is_some() {
                    return res;
                }

                properties.push((tok, expr.unwrap()));
            }

            if self.current_token.typee == Tokens::Newline {
                self.advance();
                res.register_advancement()
            }

            if self.current_token.typee != Tokens::RightCurlyBraces {
                return res.failure(Error::new(
                    "Invalid syntax",
                    pos_start,
                    self.current_token.clone().pos_end,
                    "'}', ',' was expected.",
                ));
            }

            res.register_advancement();
            self.advance();
        }

        res.success(Node::ObjectDefNode { properties })
    }
}
