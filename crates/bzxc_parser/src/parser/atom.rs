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
     * Parses a atom expression
     */
    pub(crate) fn atom(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let token = self.current_token.clone();

        if let Tokens::Int(_) | Tokens::Float(_) = token.value {
            res.register_advancement();
            self.advance();
            return res.success(Node::NumberNode {
                token: token.clone(),
            });
        } else if let Tokens::Boolean(_) = token.value {
            res.register_advancement();
            self.advance();
            return res.success(Node::BooleanNode {
                token: token.clone(),
            });
        } else if let Tokens::String(_) = token.value {
            res.register_advancement();
            self.advance();
            return res.success(Node::StringNode {
                token: token.clone(),
            });
        } else if let Tokens::Char(_) = token.value {
            res.register_advancement();
            self.advance();
            return res.success(Node::CharNode {
                token: token.clone(),
            });
        } else if let Tokens::Identifier(_) = token.value {
            let var_expr = res.register(self.var_expr());
            if res.error.is_some() {
                return res;
            }
            return res.success(var_expr.unwrap());
        } else if token.value == Tokens::LeftParenthesis {
            res.register_advancement();
            self.advance();
            let expr = res.register(self.expr());
            if res.error.is_some() {
                return res;
            }
            if self.current_token.clone().value != Tokens::RightParenthesis {
                return res.failure(Error::new(
                    "Invalid Syntax",
                    self.current_token.clone().pos_start,
                    self.current_token.clone().pos_end,
                    "Expected ')'",
                ));
            }

            res.register_advancement();
            self.advance();
            return res.success(expr.unwrap());
        } else if token.value == Tokens::LeftSquareBraces {
            let array_expr = res.register(self.array_expr());
            if res.error.is_some() {
                return res;
            }
            return res.success(array_expr.unwrap());
        } else if token.value == Tokens::LeftCurlyBraces {
            let obj_expr = res.register(self.obj_expr());
            if res.error.is_some() {
                return res;
            }
            return res.success(obj_expr.unwrap());
        } else if token.value == Tokens::Keyword("if") {
            let if_expr = res.register(self.if_expr());
            if res.error.is_some() {
                return res;
            }
            return res.success(if_expr.unwrap());
        } else if token.value == Tokens::Keyword("while") {
            let while_expr = res.register(self.while_expr());
            if res.error.is_some() {
                return res;
            }
            return res.success(while_expr.unwrap());
        } else if token.value == Tokens::Keyword("for") {
            let for_expr = res.register(self.for_expr());
            if res.error.is_some() {
                return res;
            }
            return res.success(for_expr.unwrap());
        } else if token.value == Tokens::Keyword("fun") {
            let fun_def = res.register(self.fun_def());
            if res.error.is_some() {
                return res;
            }
            return res.success(fun_def.unwrap());
        } else if token.value == Tokens::Keyword("class") {
            let class_def = res.register(self.class_def());
            if res.error.is_some() {
                return res;
            }
            return res.success(class_def.unwrap());
        } else if token.value == Tokens::Keyword("new") {
            let class_init = res.register(self.class_init());
            if res.error.is_some() {
                return res;
            }
            return res.success(class_init.unwrap());
        } else if token.value == Tokens::Keyword("soul") {
            let token = Token::new(
                Tokens::Identifier("soul"),
                self.current_token.pos_start,
                self.current_token.pos_end,
            );
            self.advance();
            res.register_advancement();

            return res.success(Node::VarAccessNode { token });
        } else if token.value == Tokens::Keyword("extern") {
            let extern_def = res.register(self.extern_def());
            if res.error.is_some() {
                return res;
            }
            return res.success(extern_def.unwrap());
        } else if let Tokens::Keyword(_) = token.value {
            self.advance();
            res.register_advancement();
            
            return res.success(Node::TypeKeyword { token });
            
        }

        res.failure(Error::new(
            "Invalid Syntax",
            token.pos_start,
            token.pos_end,
            "A Int, Float, String, Char, Keyword, Identifier, '+', '-', '(', etc was Expected",
        ))
    }
}
