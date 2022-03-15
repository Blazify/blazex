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
use bzxc_shared::{Error, Node, Token, Tokens};

impl Parser {
    /*
     * Parses a class definition
     */
    pub(crate) fn class_def(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        if self.current_token.value != Tokens::Keyword("class") {
            return res.failure(Error::new(
                "Syntax Error",
                self.current_token.pos_start,
                self.current_token.pos_end,
                "Expected 'class'",
            ));
        }

        self.advance();
        res.register_advancement();

        let name = self.current_token.clone();
        if matches!(name.value, Tokens::Identifier(_)) {
            res.register_advancement();
            self.advance();
        } else {
            return res.failure(Error::new(
                "Syntax Error",
                self.current_token.pos_start,
                self.current_token.pos_end,
                "Expected 'identifier'",
            ));
        }

        let mut constructor_def = false;
        let mut methods = vec![];
        let mut static_members = vec![];

        let mut constructor = (
            vec![],
            Box::new(Node::Statements {
                statements: vec![Node::ReturnNode {
                    value: Box::new(Some(Node::VarAccessNode {
                        token: Token::new(
                            Tokens::Identifier("soul"),
                            self.current_token.pos_start,
                            self.current_token.pos_end,
                        ),
                    })),
                }],
            }),
        );
        let mut properties = vec![];

        if self.current_token.value != Tokens::LeftCurlyBraces {
            return res.failure(Error::new(
                "Syntax Error",
                self.current_token.pos_start,
                self.current_token.pos_end,
                "Expected '{'",
            ));
        }

        self.advance();
        res.register_advancement();

        while self.current_token.value != Tokens::RightCurlyBraces {
            if self.current_token.value == Tokens::Newline {
                self.advance();
                res.register_advancement();
                continue;
            }

            let mut is_static = false;
            if self.current_token.value == Tokens::Keyword("static") {
                is_static = true;
                self.advance();
                res.register_advancement();
            }

            let statement = res.try_register(self.statement());
            if res.error.is_some() {
                return res;
            }
            if statement.is_none() {
                self.reverse(res.to_reverse_count as usize);
                continue;
            }

            match statement.clone().unwrap() {
                Node::VarAssignNode {
                    name,
                    value,
                    reassignable: _,
                } => {
                    if is_static {
                        static_members.push((name, *value));
                    } else {
                        properties.push((name, *value));
                    }
                }
                Node::FunDef {
                    name,
                    arg_tokens,
                    body_node,
                } => {
                    // if name is none and there is no constructor, then it is a constructor orelse it is a function and if static keyword is present it is static member
                    if name.is_none() {
                        if !constructor_def {
                            constructor_def = true;
                            constructor = (arg_tokens, body_node);
                        } else {
                            return res.failure(Error::new(
                                "Syntax Error",
                                self.current_token.pos_start,
                                self.current_token.pos_end,
                                "Expected '}'",
                            ));
                        }
                    } else {
                        if is_static {
                            static_members.push((
                                name.unwrap(),
                                Node::FunDef {
                                    name,
                                    arg_tokens,
                                    body_node,
                                },
                            ));
                        } else {
                            methods.push((name.unwrap(), arg_tokens, *body_node));
                        }
                    }
                }
                _ => {
                    return res.failure(Error::new(
                        "Syntax Error",
                        self.current_token.pos_start,
                        self.current_token.pos_end,
                        "Expected properties or methods",
                    ))
                }
            }
        }

        if self.current_token.value != Tokens::RightCurlyBraces {
            return res.failure(Error::new(
                "Syntax Error",
                self.current_token.pos_start,
                self.current_token.pos_end,
                "Expected '}'",
            ));
        }

        self.advance();
        res.register_advancement();

        res.success(Node::ClassDefNode {
            constructor,
            methods,
            name,
            properties,
            static_members,
        })
    }
}
