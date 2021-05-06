/*
   Copyright 2021 Blazify

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
use crate::compiler::parser::nodes::Node;
use crate::utils::error::Error;

#[derive(Debug, Clone)]
pub struct ParseResult {
    pub node: Option<Node>,
    pub error: Option<Error>,
    pub advance_count: i64,
    pub to_reverse_count: i64,
}

impl ParseResult {
    pub fn new() -> ParseResult {
        ParseResult {
            node: None,
            error: None,
            advance_count: 0,
            to_reverse_count: 0,
        }
    }

    pub fn register(&mut self, res: ParseResult) -> Option<Node> {
        self.advance_count += res.advance_count;
        if res.error.is_some() {
            self.error = res.error.clone();
        };
        res.node
    }

    pub fn try_register(&mut self, res: ParseResult) -> Option<Node> {
        if res.error.is_some() {
            self.to_reverse_count = res.advance_count;
            return None;
        };
        self.register(res)
    }

    pub fn register_advancement(&mut self) {
        self.advance_count += 1;
    }

    pub fn success(&mut self, node: Node) -> ParseResult {
        self.node = Some(node);
        self.clone()
    }

    pub fn failure(&mut self, error: Error) -> ParseResult {
        self.error = Some(error);
        self.clone()
    }
}
