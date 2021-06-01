use bzxc_shared::{DynType, Error, Token, Tokens};

use crate::Lexer;

impl Lexer {
    /*
     * Makes a number token
     */
    pub(crate) fn make_number(&mut self) -> Token {
        let mut str_num = String::new();
        let mut dot_count = 0;
        let start = self.position.clone();

        while self.current_char.is_some() {
            if self.current_char.unwrap().to_digit(36).is_none()
                && self.current_char.unwrap() != '.'
            {
                break;
            }
            if self.current_char.unwrap() == '.' {
                dot_count += 1;
            }
            str_num.push(self.current_char.unwrap());
            self.advance();
        }

        return if dot_count > 0 {
            Token::new(
                Tokens::Float,
                start,
                self.position.clone(),
                DynType::Float(str_num.parse::<f64>().unwrap()),
            )
        } else {
            Token::new(
                Tokens::Int,
                start,
                self.position.clone(),
                DynType::Int(str_num.parse::<i128>().unwrap()),
            )
        };
    }

    /*
     * Makes a String Token
     */
    pub(crate) fn make_string(&mut self) -> Token {
        let mut str_raw = String::new();
        let start = self.position.clone();
        let mut escape = false;
        self.advance();

        while self.current_char.is_some() || escape {
            if self.current_char.unwrap() == '"' {
                break;
            }
            if escape {
                if self.current_char.unwrap() == 'n' {
                    str_raw.push('\n');
                } else if self.current_char.unwrap() == 't' {
                    str_raw.push('\t');
                } else {
                    str_raw.push(self.current_char.unwrap());
                }
            } else {
                if self.current_char.unwrap() == '\\' {
                    escape = true;
                    self.advance();
                    continue;
                } else {
                    str_raw.push(self.current_char.unwrap());
                }
            }

            self.advance();
            escape = false;
        }

        self.advance();

        Token::new(
            Tokens::String,
            start,
            self.position.clone(),
            DynType::String(str_raw),
        )
    }

    /*
     * Makes a charecter token
     */
    pub(crate) fn make_char(&mut self) -> Result<Token, Error> {
        let start = self.position.clone();

        self.advance();
        let new_char = self.current_char;
        self.advance();

        if self.current_char.unwrap_or(' ') != '\'' {
            return Err(Error::new(
                "Expected Character",
                start,
                self.position.clone(),
                "Expected Character \"'\" because chars are unicode characters.",
            ));
        }

        self.advance();

        Ok(Token::new(
            Tokens::Char,
            start,
            self.position.clone(),
            DynType::Char(new_char.unwrap()),
        ))
    }
}
