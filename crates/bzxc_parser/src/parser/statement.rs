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
use bzxc_shared::{DynType, Node, Tokens};

impl Parser {
    /*
     * Parse a statement
     */
    pub(crate) fn statement(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        if self
            .clone()
            .current_token
            .matches(Tokens::Keyword, DynType::String("return".to_string()))
        {
            res.register_advancement();
            self.advance();

            let expr = res.try_register(self.expr());
            if expr.is_none() {
                self.reverse(res.to_reverse_count as usize);
            }

            return res.success(Node::ReturnNode {
                value: Box::new(expr),
            });
        }

        let expr = res.register(self.expr());
        if res.error.is_some() {
            return res;
        }
        res.success(expr.unwrap())
    }
}
