use crate::core::parser::nodes::Node;
use crate::utils::error::Error;

#[derive(Debug, Clone)]
pub struct ParseResult {
    pub node: Option<Node>,
    pub error: Option<Error>,
    pub advance_count: i64,
}

impl ParseResult {
    pub fn new() -> ParseResult {
        ParseResult {
            node: None,
            error: None,
            advance_count: 0,
        }
    }

    pub fn register(&mut self, res: ParseResult) -> Option<Node> {
        self.advance_count += res.advance_count;
        if res.error.is_some() {
            self.error = res.error.clone();
        };
        res.node
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
