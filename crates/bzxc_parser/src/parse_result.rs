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

use bzxc_shared::{Error, Node};

/*
* Result returned after statement(s) are parsed by the error
*/
#[derive(Debug, Clone)]
pub struct ParseResult {
    pub node: Option<Node>,
    pub error: Option<Error>,
    pub advance_count: usize,
    pub to_reverse_count: usize,
}

impl ParseResult {
    /*
     * Creates a new instance of Parse Result
     */
    pub fn new() -> ParseResult {
        ParseResult {
            node: None,
            error: None,
            advance_count: 0,
            to_reverse_count: 0,
        }
    }

    /*
     * Registers node and error of a result into the current
     */
    pub fn register(&mut self, res: ParseResult) -> Option<Node> {
        self.advance_count += res.advance_count;
        if res.error.is_some() {
            self.error = res.error.clone();
        };
        res.node
    }

    /*
     * Register a Result if there is no error
     */
    pub fn try_register(&mut self, res: ParseResult) -> Option<Node> {
        if res.error.is_some() {
            self.to_reverse_count = res.advance_count;
            return None;
        };
        self.register(res)
    }

    /*
     * Advance the Node Array Index by one
     */
    pub fn register_advancement(&mut self) {
        self.advance_count += 1;
    }

    /*
     * Return a Result with a node
     */
    pub fn success(&mut self, node: Node) -> ParseResult {
        self.node = Some(node);
        self.clone()
    }

    /*
     * Return a Result with a error
     */
    pub fn failure(&mut self, error: Error) -> ParseResult {
        self.error = Some(error);
        self.clone()
    }
}
