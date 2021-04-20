use crate::utils::{context::Context, position::Position};

#[derive(Debug, Clone)]
pub struct Error {
    pub name: &'static str,
    pub pos_start: Position,
    pub pos_end: Position,
    pub description: &'static str,
    pub ctx: Option<Context>,
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
            ctx: None,
        }
    }

    pub fn set_ctx(mut self, ctx: Context) -> Self {
        self.ctx = Some(ctx);
        self
    }

    pub fn prettify(&mut self) -> String {
        if self.ctx.is_some() {
            let mut res = String::new();
            let mut pos = Some(self.pos_start);
            let mut ctx = self.ctx.as_ref();

            while ctx.is_some() {
                let file_name = if pos.is_none() {
                    "Unknown"
                } else {
                    pos.unwrap().file_name
                };

                let line = if pos.is_none() { 0 } else { pos.unwrap().line };

                res = format!(
                    "File {}, line: {}, in {}\n{}",
                    file_name,
                    line,
                    ctx.unwrap().display_name,
                    res
                );

                pos = ctx.unwrap().parent_position;
                if ctx.unwrap().parent.is_some() {
                    ctx = ctx.unwrap().parent.as_ref().as_ref();
                }
            }

            return format!("Traceback (most recent call last):\n{}", res.clone());
        }

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
            " ".repeat((self.pos_start.column) as usize)
                + &*"^".repeat((self.pos_end.column - self.pos_start.column + 1) as usize)
        )
    }
}
