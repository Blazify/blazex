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
use bzs_shared::{DynType, Error, Node, Token, Tokens};

impl Parser {
    /*
     * Parses a class definition
     */
    pub(crate) fn class_def(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let mut methods: Vec<(Token, Vec<Token>, Node)> = vec![];
        let mut properties: Vec<(Token, Node)> = vec![];
        let mut constructor: Option<(Vec<Token>, Node)> = None;

        if !self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("class".to_string()))
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected 'class'",
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

        if self.current_token.typee != Tokens::LeftCurlyBraces {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '{'",
            ));
        }

        res.register_advancement();
        self.advance();

        while self.current_token.typee != Tokens::RightCurlyBraces {
            while self.current_token.typee == Tokens::Newline {
                res.register_advancement();
                self.advance();
            }
            if self.current_token.typee == Tokens::RightCurlyBraces {
                break;
            }
            let stnts = res.register(self.expr());
            if res.error.is_some() {
                return res;
            }
            let a = stnts.unwrap();
            match a.clone() {
                Node::VarAssignNode { name, value, .. } => {
                    properties.push((name.clone(), *value.clone()))
                }
                Node::FunDef {
                    name,
                    body_node,
                    arg_tokens,
                    return_type: _,
                } => {
                    if name.as_ref().is_none() {
                        if constructor.is_some() {
                            return res.failure(Error::new(
                                "Invalid Syntax",
                                self.current_token.pos_start.clone(),
                                self.current_token.pos_end.clone(),
                                "Constructor defined",
                            ));
                        }
                        constructor = Some((
                            arg_tokens.iter().map(|x| x.clone().0).collect(),
                            *body_node.clone(),
                        ));
                    } else {
                        methods.push((
                            name.as_ref().unwrap().clone(),
                            arg_tokens.iter().map(|x| x.clone().0).collect(),
                            *body_node,
                        ));
                    }
                }
                _ => {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected properties or methods",
                    ))
                }
            }
        }

        if self.current_token.typee != Tokens::RightCurlyBraces {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '}'",
            ));
        }

        res.register_advancement();
        self.advance();

        res.success(Node::ClassDefNode {
            name,
            constructor: Box::new(constructor),
            properties,
            methods,
        })
    }
}
