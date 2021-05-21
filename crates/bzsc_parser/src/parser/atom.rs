/*
   Copyright 2021 BlazifyOrg
   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at
       http://www.apache.org/licenses/LICENSE-2.0
   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use super::Parser;
use crate::parse_result::ParseResult;
use bzs_shared::{DynType, Error, Node, Tokens};

impl Parser {
    /*
     * Parses a atom expression
     */
    pub(crate) fn atom(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let token = self.current_token.clone();

        if [Tokens::Int, Tokens::Float].contains(&token.r#type) {
            res.register_advancement();
            self.advance();
            return res.success(Node::NumberNode {
                token: token.clone(),
            });
        } else if token.r#type == Tokens::Boolean {
            res.register_advancement();
            self.advance();
            return res.success(Node::BooleanNode {
                token: token.clone(),
            });
        } else if token.r#type == Tokens::String {
            res.register_advancement();
            self.advance();
            return res.success(Node::StringNode {
                token: token.clone(),
            });
        } else if token.r#type == Tokens::Char {
            res.register_advancement();
            self.advance();
            return res.success(Node::CharNode {
                token: token.clone(),
            });
        } else if token.r#type == Tokens::Identifier {
            res.register_advancement();
            self.advance();

            if self.current_token.r#type == Tokens::Equals {
                res.register_advancement();
                self.advance();

                let new_value = res.register(self.expr());
                if res.error.is_some() {
                    return res;
                }

                return res.success(Node::VarReassignNode {
                    name: token.clone(),
                    value: Box::new(new_value.clone().unwrap()),
                });
            }

            return res.success(Node::VarAccessNode {
                token: token.clone(),
            });
        } else if token.r#type == Tokens::LeftParenthesis {
            res.register_advancement();
            self.advance();
            let expr = res.register(self.expr());
            if res.error.is_some() {
                return res;
            }
            if self.current_token.clone().r#type != Tokens::RightParenthesis {
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
        } else if token.r#type == Tokens::LeftSquareBraces {
            let array_expr = res.register(self.array_expr());
            if res.error.is_some() {
                return res;
            }
            return res.success(array_expr.unwrap());
        } else if token.r#type == Tokens::LeftCurlyBraces {
            let obj_expr = res.register(self.obj_expr());
            if res.error.is_some() {
                return res;
            }
            return res.success(obj_expr.unwrap());
        } else if token
            .clone()
            .matches(Tokens::Keyword, DynType::String("if".to_string()))
        {
            let if_expr = res.register(self.if_expr());
            if res.error.is_some() {
                return res;
            }
            return res.success(if_expr.unwrap());
        } else if token
            .clone()
            .matches(Tokens::Keyword, DynType::String("while".to_string()))
        {
            let while_expr = res.register(self.while_expr());
            if res.error.is_some() {
                return res;
            }
            return res.success(while_expr.unwrap());
        } else if token
            .clone()
            .matches(Tokens::Keyword, DynType::String("for".to_string()))
        {
            let for_expr = res.register(self.for_expr());
            if res.error.is_some() {
                return res;
            }
            return res.success(for_expr.unwrap());
        } else if token
            .clone()
            .matches(Tokens::Keyword, DynType::String("fun".to_string()))
        {
            let fun_def = res.register(self.fun_def());
            if res.error.is_some() {
                return res;
            }
            return res.success(fun_def.unwrap());
        } else if token
            .clone()
            .matches(Tokens::Keyword, DynType::String("class".to_string()))
        {
            let class_def = res.register(self.class_def());
            if res.error.is_some() {
                return res;
            }
            return res.success(class_def.unwrap());
        } else if token
            .clone()
            .matches(Tokens::Keyword, DynType::String("new".to_string()))
        {
            let class_init = res.register(self.class_init());
            if res.error.is_some() {
                return res;
            }
            return res.success(class_init.unwrap());
        }

        res.failure(Error::new(
            "Invalid Syntax",
            token.pos_start,
            token.pos_end,
            "A Int, Float, String, Char, Keyword, Identifier, '+', '-', '(', etc was Expected",
        ))
    }
}
