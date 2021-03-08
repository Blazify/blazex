use crate::utils::position::Position;

#[derive(Debug, Clone)]
pub struct Error {
    pub name: String,
    pub pos_start: Position,
    pub pos_end: Position,
    pub description: String,
}

impl Error {
    pub fn new(name: &str, pos_start: Position, pos_end: Position, description: &str) -> Error {
        Error {
            name: String::from(name),
            pos_start,
            pos_end,
            description: String::from(description)
        }
    }

    pub fn prettify(&mut self) -> String {
        format!(
            "\u{001b}[31;1m{}: {}\nFile {}, line {}\n\n {}\n {}\u{001b}[0m",
            self.name,
            self.description,
            self.pos_start.file_name,
            self.pos_start.line + 1,
            self.pos_start
                .file_content
                .to_string()
                .split("\n")
                .collect::<Vec<&str>>()[self.pos_start.line as usize]
                .replace("\t", ""),
            " ".repeat((self.pos_start.column - 1) as usize)
                + &*"^".repeat((self.pos_end.column - self.pos_start.column + 1) as usize)
        )
    }
}
