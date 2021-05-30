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
use bzc_shared::{Node, Tokens};

impl Parser {
    /*
     * Parse factor expressions
     */
    pub(crate) fn factor(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let token = self.current_token.clone();

        if [Tokens::Plus, Tokens::Minus].contains(&self.current_token.typee) {
            res.register_advancement();
            self.advance();
            let factor = res.register(self.factor());
            if res.error.is_some() {
                return res;
            }
            return res.success(Node::UnaryNode {
                op_token: token.clone(),
                node: Box::new(factor.clone().unwrap()),
            });
        }
        self.power()
    }
}
