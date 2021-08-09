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

impl Parser {
    /*
     * Parses a object property expression
     */
    pub(crate) fn obj_prop_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let expr = res.register(self.index_expr());
        if res.error.is_some() {
            return res;
        }

        self.call_access_expr(expr, res)
    }
}
