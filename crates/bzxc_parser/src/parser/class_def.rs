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
use bzxc_shared::{Error, Node, Token, Tokens, Type};

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

        let name = self.current_token;
        if let Tokens::Identifier(_) = self.current_token.value {
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
        let mut methods: Vec<(Token, Vec<(Token, Type)>, Node, Type)> = vec![];
        let mut constructor: (Vec<(Token, Type)>, Box<Node>) =
            (vec![], Box::new(Node::Statements { statements: vec![] }));
        let mut properties: Vec<(Token, Node)> = vec![];

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

        let statements_node = res.register(self.statements());
        if res.error.is_some() {
            return res;
        }

        if let Node::Statements { statements } = statements_node.unwrap() {
            for statement in statements {
                match statement {
                    Node::VarAssignNode {
                        name,
                        value,
                        reassignable: _,
                    } => {
                        properties.push((name, *value));
                    }
                    Node::FunDef {
                        name,
                        arg_tokens,
                        body_node,
                        return_type,
                    } => {
                        if name.is_none() {
                            if constructor_def {
                                return res.failure(Error::new(
                                    "Syntax Error",
                                    self.current_token.pos_start,
                                    self.current_token.pos_end,
                                    "Constructor already defined",
                                ));
                            }

                            constructor_def = true;
                            constructor = (arg_tokens, body_node);
                        } else {
                            methods.push((name.unwrap(), arg_tokens, *body_node, return_type))
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
        })
    }
}
