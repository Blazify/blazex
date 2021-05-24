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
use bzs_shared::{Error, Node, Tokens};

impl Parser {
    /*
     * Parses a array
     */
    pub(crate) fn array_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let mut element_nodes: Vec<Node> = vec![];
        let token = self.current_token.clone();
        let pos_start = self.current_token.pos_start.clone();

        if self.current_token.typee != Tokens::LeftSquareBraces {
            return res.failure(Error::new(
                "Invalid syntax",
                pos_start,
                token.pos_end,
                "'[' was expected.",
            ));
        }

        res.register_advancement();
        self.advance();

        if self.current_token.typee == Tokens::RightSquareBraces {
            res.register_advancement();
            self.advance();
        } else {
            let mut expr = res.register(self.expr());
            if res.error.is_some() {
                return res.failure(Error::new(
                    "Invalid Syntax",
                    pos_start,
                    token.pos_end,
                    "Expected ']', 'var', 'if', 'for', 'while', 'fun', int, float, identifier, '+', '-', '(', '[' or 'NOT'"
                ));
            }

            element_nodes.push(expr.unwrap());
            while self.current_token.typee == Tokens::Comma {
                res.register_advancement();
                self.advance();

                expr = res.register(self.expr());
                if res.error.is_some() {
                    return res;
                }
                element_nodes.push(expr.unwrap());
            }

            if self.current_token.typee != Tokens::RightSquareBraces {
                return res.failure(Error::new(
                    "Invalid Syntax",
                    pos_start,
                    token.pos_end,
                    "Expected ']' or ','.",
                ));
            }
            res.register_advancement();
            self.advance();
        }

        res.success(Node::ArrayNode { element_nodes })
    }
}
