use bzxc_shared::{Error, Token, Tokens};

use crate::Lexer;

impl Lexer {
    /*
     * Makes a DOUBLE_EQUALS | ARROW | EQUALS Token
     */
    pub(crate) fn make_equals(&mut self) -> Token {
        let start = self.position.clone();
        self.advance();

        if self.current_char.unwrap_or(' ') == '=' {
            self.advance();
            return Token::new(Tokens::DoubleEquals, start, self.position.clone());
        }

        Token::new(Tokens::Equals, start, self.position.clone())
    }

    /*
     * Makes a LESS_THAN or LESS_THAN_EQUALS Token
     */
    pub(crate) fn make_less_than(&mut self) -> Token {
        let start = self.position.clone();
        self.advance();

        if self.current_char.unwrap_or(' ') == '=' {
            self.advance();
            return Token::new(Tokens::LessThanEquals, start, self.position.clone());
        }

        Token::new(Tokens::LessThan, start, self.position.clone())
    }

    /*
     * Makes a GREATER_THAN or GREATER_THAN_EQUALS Token
     */
    pub(crate) fn make_greater_than(&mut self) -> Token {
        let start = self.position.clone();
        self.advance();

        if self.current_char.unwrap_or(' ') == '=' {
            self.advance();
            return Token::new(Tokens::GreaterThanEquals, start, self.position.clone());
        }

        Token::new(Tokens::GreaterThan, start, self.position.clone())
    }

    /*
     * Makes a NOT or NOT_EQUALS Token
     */
    pub(crate) fn make_not(&mut self) -> Token {
        let start = self.position.clone();
        self.advance();

        if self.current_char.unwrap_or(' ') == '=' {
            self.advance();
            return Token::new(Tokens::NotEquals, start, self.position.clone());
        }

        Token::new(Tokens::Keyword("not"), start, self.position.clone())
    }

    /*
     * Makes a NOT Token
     */
    pub(crate) fn make_or(&mut self) -> Result<Token, Error> {
        let start = self.position.clone();
        self.advance();

        if self.current_char.unwrap_or(' ') == '|' {
            self.advance();
            return Ok(Token::new(
                Tokens::Keyword("or"),
                start,
                self.position.clone(),
            ));
        }

        Err(Error::new(
            "Expected Character",
            start,
            self.position.clone(),
            "Expected one more '|'",
        ))
    }

    /*
     * Makes a AND Token
     */
    pub(crate) fn make_and(&mut self) -> Result<Token, Error> {
        let start = self.position.clone();
        self.advance();

        if self.current_char.unwrap_or(' ') == '&' {
            self.advance();
            return Ok(Token::new(
                Tokens::Keyword("and"),
                start,
                self.position.clone(),
            ));
        }

        Err(Error::new(
            "Expected Character",
            start,
            self.position.clone(),
            "Expected one more '&'",
        ))
    }
}
