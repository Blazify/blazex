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
use crate::utils::position::Position;

#[derive(Debug, Clone)]
pub struct Error {
    pub name: &'static str,
    pub pos_start: Position,
    pub pos_end: Position,
    pub description: &'static str,
}

impl Error {
    pub fn new(
        name: &'static str,
        pos_start: Position,
        pos_end: Position,
        description: &'static str,
    ) -> Error {
        Error {
            name,
            pos_start,
            pos_end,
            description,
        }
    }

    pub fn prettify(&self) -> String {
        format!(
            "\u{001b}[31;1m{}: {}\nFile {}, line {}\n\n {}\u{001b}[0m",
            self.name,
            self.description,
            self.pos_start.file_name,
            self.pos_start.line + 1,
            self.string_with_arrows(),
        )
    }

    fn string_with_arrows(&self) -> String {
        let mut res = String::new();
        let text = self.pos_start.file_content.to_string().clone();

        let mut idx_start = std::cmp::max(
            text[0..self.pos_start.index as usize]
                .rfind("\n")
                .unwrap_or(0),
            0,
        );
        let mut idx_end = text[(idx_start + 1)..(text.len() - 1)]
            .find("\n")
            .unwrap_or(0);
        if idx_end < 0 as usize {
            idx_end = text.len();
        }
        let line_count = self.pos_end.line - self.pos_start.line + 1;

        for i in 0..line_count {
            let line = &text[idx_start..(idx_end + idx_start)];

            let mut col_start = 0;
            if i == 0 {
                col_start = self.pos_start.column;
            }

            let mut col_end = line.len() as i64 - 1;
            if i == (line_count - 1) {
                col_end = self.pos_end.column;
            }

            res.push_str(line);
            res.push('\n');
            res = format!(
                "{}{}",
                res,
                " ".repeat((col_start) as usize) + &*"^".repeat((col_end - col_start) as usize)
            );

            idx_start = idx_end;
            idx_end = text[(idx_start + 1)..(text.len() - 1)]
                .find("\n")
                .unwrap_or(0);
            if idx_end < 0 as usize {
                idx_end = text.len();
            }
        }

        res.replacen("\t", "", res.len())
    }
}
