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

use crate::parse_result::ParseResult;
use bzs_shared::{Error, Token, Tokens};

mod arith_expr;
mod array_expr;
mod atom;
mod call;
mod class_def;
mod class_init;
mod comp_expr;
mod expr;
mod factor;
mod for_expr;
mod fun_def;
mod if_expr;
mod index_expr;
mod obj_expr;
mod obj_prop_expr;
mod power;
mod statement;
mod statements;
mod term;
mod while_expr;

/*
* Parses Tokens into a Statements Node with child nodes
*/
#[derive(Debug, Clone)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub token_index: usize,
    pub current_token: Token,
}

impl Parser {
    /*
     * Creates a new Parser instance
     */
    pub fn new(tokens: Vec<Token>) -> Parser {
        let current_token = tokens.clone()[0].clone();
        Parser {
            tokens,
            token_index: 0,
            current_token,
        }
    }

    /*
     * Parses tokens into a node
     */
    pub fn parse(&mut self) -> ParseResult {
        let mut res = self.statements();
        if res.error.is_none() && self.current_token.r#type != Tokens::EOF {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected Operators, Variables, Functions, etc but found none",
            ));
        }
        res
    }

    /*
     * Advances to the next token
     */
    fn advance(&mut self) -> Token {
        self.token_index += 1;
        self.update_current_token();
        self.current_token.clone()
    }

    /*
     * Updates the current token based upon the token index
     */
    fn update_current_token(&mut self) {
        if self.token_index >= 0 as usize && self.token_index < self.tokens.len() {
            self.current_token = self.tokens.clone()[self.clone().token_index].clone();
        }
    }

    /*
     * Reverse tokens by provided offset
     */
    fn reverse(&mut self, cnt: usize) -> Token {
        self.token_index -= cnt;
        self.update_current_token();

        self.clone().current_token
    }
}
