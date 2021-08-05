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
     * Parses a function definition
     */
    pub(crate) fn fun_def(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        if self.current_token.value != Tokens::Keyword("fun") {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected 'fun'",
            ));
        }

        res.register_advancement();
        self.advance();

        let mut fun_name: Option<Token> = None;
        if let Tokens::Identifier(_) = self.current_token.value {
            fun_name = Some(self.current_token.clone());

            res.register_advancement();
            self.advance();

            if self.current_token.value != Tokens::LeftParenthesis {
                return res.failure(Error::new(
                    "Invalid Syntax",
                    self.current_token.pos_start.clone(),
                    self.current_token.pos_end.clone(),
                    "Expected '('",
                ));
            }
        } else if self.current_token.value != Tokens::LeftParenthesis {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '(' or identifier",
            ));
        }

        res.register_advancement();
        self.advance();

        let mut args_name_tokens: Vec<(Token, Type)> = vec![];
        if self.is_type_decl() {
            let name = self.current_token.clone();
            res.register_advancement();
            self.advance();
            if self.current_token.value != Tokens::Colon {
                return res.failure(Error::new(
                    "Invalid Syntax",
                    self.current_token.pos_start.clone(),
                    self.current_token.pos_end.clone(),
                    "Expected ':'",
                ));
            }

            res.register_advancement();
            self.advance();
            let typee = self.type_expr(&mut res);
            match typee {
                Ok(typ) => args_name_tokens.push((name, typ)),
                Err(e) => return res.failure(e),
            }

            while self.current_token.value == Tokens::Comma {
                res.register_advancement();
                self.advance();

                if self.is_type_decl() {
                    let new_arg_token = self.current_token.clone();
                    res.register_advancement();
                    self.advance();
                    if self.current_token.value != Tokens::Colon {
                        return res.failure(Error::new(
                            "Invalid Syntax",
                            self.current_token.pos_start.clone(),
                            self.current_token.pos_end.clone(),
                            "Expected ':'",
                        ));
                    }

                    res.register_advancement();
                    self.advance();
                    let typee = self.type_expr(&mut res);
                    match typee {
                        Ok(typ) => args_name_tokens.push((new_arg_token, typ)),
                        Err(e) => return res.failure(e),
                    }
                } else {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected Identifier",
                    ));
                }
            }

            if self.current_token.value != Tokens::RightParenthesis {
                return res.failure(Error::new(
                    "Invalid Syntax",
                    self.current_token.pos_start.clone(),
                    self.current_token.pos_end.clone(),
                    "Expected ')' or ','",
                ));
            }
        } else if self.current_token.value != Tokens::RightParenthesis {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected ')' or identifier",
            ));
        }

        res.register_advancement();
        self.advance();
        if self.current_token.value != Tokens::Colon {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected ':'",
            ));
        }

        res.register_advancement();
        self.advance();
        let ret_type = self.type_expr(&mut res);
        let return_type = match ret_type {
            Ok(ret) => ret,
            Err(e) => return res.failure(e),
        };

        if self.current_token.value != Tokens::LeftCurlyBraces {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '{'",
            ));
        }
        self.advance();
        res.register_advancement();

        let body_node = res.register(self.statements());
        if res.error.is_some() {
            return res;
        }

        if self.current_token.value != Tokens::RightCurlyBraces {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '}'",
            ));
        }
        self.advance();
        res.register_advancement();

        res.success(Node::FunDef {
            name: fun_name,
            body_node: Box::new(body_node.clone().unwrap()),
            arg_tokens: args_name_tokens,
            return_type,
        })
    }
}
