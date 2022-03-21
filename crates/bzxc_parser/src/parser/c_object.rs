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
use bzxc_shared::{Error, Node, Tokens};

impl Parser {
    pub(crate) fn c_object(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        if self.current_token.value != Tokens::Keyword("CObject") {
            return res.failure(Error::new(
                "Syntax error",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected CObject",
            ));
        }

        self.advance();
        res.register_advancement();

        if self.current_token.value != Tokens::LeftParenthesis {
            return res.failure(Error::new(
                "Syntax error",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected (",
            ));
        }

        self.advance();
        res.register_advancement();

        let obj = res.register(self.expr());
        if res.error.is_some() {
            return res;
        }

        if self.current_token.value != Tokens::RightParenthesis {
            return res.failure(Error::new(
                "Syntax error",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected )",
            ));
        }

        res.register_advancement();
        self.advance();

        res.success(
            Node::CObject {
                object: Box::new(obj.unwrap()),
            }
        )
    }
}
