use crate::core::token::Token;
use crate::utils::error::Error;

#[derive(Debug)]
pub struct LexerMethodResult {
    pub token: Option<Token>,
    pub error: Option<Error>,
}

impl LexerMethodResult {
    pub fn new(token: Option<Token>, error: Option<Error>) -> LexerMethodResult {
        LexerMethodResult { token, error }
    }
}
