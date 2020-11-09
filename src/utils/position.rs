#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub index: i64,
    pub line: i64,
    pub column: i64,
    pub file_name: &'static str,
    pub file_content: &'static str,
}

impl Position {
    pub fn new(
        index: i64,
        line: i64,
        column: i64,
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

    pub fn advance(&mut self, charecter: char) {
        self.index += 1;
        self.column += 1;
        if charecter == '\n' {
            self.line += 1;
            self.column += 1;
        }
    }

    pub fn clone(&mut self) -> Position {
        Position {
            index: self.index,
            line: self.index,
            column: self.column,
            file_name: self.file_name,
            file_content: self.file_content,
        }
    }
}
