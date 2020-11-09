use crate::core::token::Token;
use crate::utils::error::Error;

#[derive(Debug)]
pub struct LexerResult {
    pub tokens: Vec<Token>,
    pub error: Option<Error>,
}

impl LexerResult {
    pub fn new(tokens: Option<Vec<Token>>, error: Option<Error>) -> LexerResult {
        LexerResult {
            tokens: tokens.unwrap_or(vec![]),
            error,
        }
    }
}
