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
     * Parse a extern function declaration
     */
    pub(crate) fn extern_def(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let pos_start = self.current_token.pos_start.clone();

        if self.current_token.value != Tokens::Keyword("extern") {
            return res.failure(
                Error::new(
                    pos_start.file_name,
                    pos_start,
                    self.current_token.pos_end,
                    "Expected extern keyword",
                )
            )
        }

        self.advance();
        res.register_advancement();

        let expr = res.register(self.expr());
        if res.error.is_some() {
            return res;
        }

        let name = self.current_token.clone();
        if matches!(self.current_token.value, Tokens::Identifier(_)) {
            self.advance();
            res.register_advancement();
        } else {
            return res.failure(
                Error::new(
                    pos_start.file_name,
                    pos_start,
                    self.current_token.pos_end,
                    "Expected identifier",
                )
            )
        }

        if self.current_token.value != Tokens::LeftParenthesis {
            return res.failure(
                Error::new(
                    pos_start.file_name,
                    pos_start,
                    self.current_token.pos_end,
                    "Expected (",
                )
            )
        }

        self.advance();
        res.register_advancement();

        let mut args = Vec::new();
        let mut var_args = false;
        while self.current_token.value != Tokens::RightParenthesis {
            if self.current_token.value == Tokens::Dot {
                self.advance();
                res.register_advancement();

                if self.current_token.value != Tokens::Dot {
                    return res.failure(
                        Error::new(
                            pos_start.file_name,
                            pos_start,
                            self.current_token.pos_end,
                            "Expected .",
                        )
                    )
                }

                self.advance();
                res.register_advancement();

                if self.current_token.value != Tokens::Dot {
                    return res.failure(
                        Error::new(
                            pos_start.file_name,
                            pos_start,
                            self.current_token.pos_end,
                            "Expected .",
                        )
                    )
                }

                self.advance();
                res.register_advancement();

                var_args = true;
                break;
            }

            let arg = res.register(self.expr());
            if res.error.is_some() {
                return res;
            }
            args.push(arg.unwrap());

            if self.current_token.value != Tokens::Comma {
                break;
            }

            self.advance();
            res.register_advancement();
        }


        if self.current_token.value != Tokens::RightParenthesis {
            return res.failure(
                Error::new(
                    pos_start.file_name,
                    pos_start,
                    self.current_token.pos_end,
                    "Expected )",
                )
            )
        }

        res.register_advancement();
        self.advance();


        res.success(Node::ExternNode {
            name,
            arg_tokens: args,
            return_type: Box::new(expr.unwrap()),
            var_args
        })
    }
}
