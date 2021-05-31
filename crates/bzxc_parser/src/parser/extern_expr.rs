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

use bzxc_shared::{DynType, Error, Node, Tokens, Type};

use crate::parse_result::ParseResult;

use super::Parser;

impl Parser {
    pub(crate) fn extern_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let pos_start = self.current_token.pos_start.clone();

        if !self
            .current_token
            .matches(Tokens::Keyword, DynType::String("extern".to_string()))
        {
            return res.failure(Error::new(
                "Syntax Error",
                pos_start,
                self.current_token.pos_end.clone(),
                "Expected 'extern'",
            ));
        }

        self.advance();
        res.register_advancement();

        let var_args = if self
            .current_token
            .matches(Tokens::Keyword, DynType::String("variadic".to_string()))
        {
            self.advance();
            res.register_advancement();
            true
        } else {
            false
        };

        if !self
            .current_token
            .matches(Tokens::Keyword, DynType::String("fun".to_string()))
        {
            return res.failure(Error::new(
                "Syntax Error",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected 'fun'",
            ));
        }

        self.advance();
        res.register_advancement();

        if self.current_token.typee != Tokens::Identifier {
            return res.failure(Error::new(
                "Syntax Error",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected identifier",
            ));
        }

        let name = self.current_token.clone();
        self.advance();
        res.register_advancement();

        if self.current_token.typee != Tokens::LeftParenthesis {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '('",
            ));
        }

        res.register_advancement();
        self.advance();

        let mut args_name_tokens: Vec<Type> = vec![];
        if self.current_token.typee == Tokens::Keyword {
            let typee = self.type_expr(&mut res);
            match typee {
                Ok(typ) => args_name_tokens.push(typ),
                Err(e) => return res.failure(e),
            }

            while self.current_token.typee == Tokens::Comma {
                res.register_advancement();
                self.advance();

                if self.current_token.typee == Tokens::Keyword {
                    let typee = self.type_expr(&mut res);
                    match typee {
                        Ok(typ) => args_name_tokens.push(typ),
                        Err(e) => return res.failure(e),
                    }
                } else {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected keyword",
                    ));
                }
            }

            if self.current_token.typee != Tokens::RightParenthesis {
                return res.failure(Error::new(
                    "Invalid Syntax",
                    self.current_token.pos_start.clone(),
                    self.current_token.pos_end.clone(),
                    "Expected ')' or ','",
                ));
            }
        } else if self.current_token.typee != Tokens::RightParenthesis {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected ')' or keyword",
            ));
        }

        res.register_advancement();
        self.advance();

        if self.current_token.typee != Tokens::Colon {
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

        res.success(Node::ExternNode {
            name,
            arg_tokens: args_name_tokens,
            return_type,
            var_args,
        })
    }
}
