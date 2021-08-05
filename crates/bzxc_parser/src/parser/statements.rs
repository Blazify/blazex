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
use bzxc_shared::Node;
use bzxc_shared::Tokens;

use super::Parser;

impl Parser {
    /*
     * Parse Statements
     */
    pub(crate) fn statements(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let mut statements: Vec<Node> = vec![];

        while self.current_token.value == Tokens::Newline {
            res.register_advancement();
            self.advance();
        }

        let mut statement = res.register(self.statement());
        if res.error.is_some() {
            return res;
        };
        statements.push(statement.unwrap());
        let mut more_statements = true;

        loop {
            let mut newline_ct = 0;
            while self.current_token.value == Tokens::Newline {
                res.register_advancement();
                self.advance();
                newline_ct += 1;
            }

            if newline_ct == 0 {
                more_statements = false;
            }

            if !more_statements {
                break;
            }
            statement = res.try_register(self.statement());
            if statement.is_none() {
                self.reverse(res.to_reverse_count as usize);
                more_statements = false;
                continue;
            }
            statements.push(statement.unwrap())
        }
        res.success(Node::Statements { statements })
    }
}
