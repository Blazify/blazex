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
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub index: i128,
    pub line: i128,
    pub column: i128,
    pub file_name: &'static str,
    pub file_content: &'static str,
}

impl Position {
    pub fn new(
        index: i128,
        line: i128,
        column: i128,
        file_name: &'static str,
        file_content: &'static str,
    ) -> Position {
        Position {
            index,
            line,
            column,
            file_name,
            file_content,
        }
    }

    pub fn advance(&mut self, character: char) -> Self {
        self.index += 1;
        self.column += 1;
        if character == '\n' {
            self.line += 1;
            self.column += 1;
        }
        self.clone()
    }
}
