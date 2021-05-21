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
use bzs_shared::{Node, Tokens};

impl Parser {
    /*
     * Parse a arithmetic expression
     */
    pub(crate) fn arith_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        let mut left = res.register(self.term());
        if res.error.is_some() {
            return res;
        }

        while [Tokens::Plus, Tokens::Minus].contains(&self.current_token.r#type) {
            let op_token = self.current_token.clone();
            res.register_advancement();
            self.advance();

            let right = res.register(self.term());
            if res.error.is_some() {
                return res;
            }

            left = Option::from(Node::BinOpNode {
                left: Box::new(left.clone().unwrap()),
                right: Box::new(right.clone().unwrap()),
                op_token,
            });
        }

        res.success(left.unwrap())
    }
}
