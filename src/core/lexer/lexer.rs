use crate::core::lexer::{lexer_method_result::LexerMethodResult, lexer_result::LexerResult};
use crate::core::token::Token;
use crate::utils::{
    constants::{
        get_ascii_letters, get_ascii_letters_and_digits, get_keywords, get_number, DynType, Tokens,
    },
    error::Error,
    position::Position,
};
use std::convert::TryInto;

pub struct Lexer {
    pub file_name: String,
    pub text: String,
    pub current_char: char,
    pub position: Position,
}

impl Lexer {
    pub fn new(file_name: &str, text: &str) -> Lexer {
        let mut lexer = Lexer {
            file_name: String::from(file_name),
            text: String::from(text),
            current_char: ' ',
            position: Position::new(-1, 0, -1, file_name, text),
        };
        lexer.advance();
        lexer
    }

    fn advance(&mut self) {
        self.position.advance(self.current_char);
        if self.text.len() > self.position.index.try_into().unwrap() {
            let split: Vec<char> = self.text.chars().collect::<Vec<char>>();
            let index: i64 = self.position.index.try_into().unwrap();
            self.current_char = split[index as usize];
        } else {
            self.current_char = ';';
        }
    }

    pub fn tokenize(&mut self) -> LexerResult {
        let mut tokens: Vec<Token> = vec![];

        while self.current_char != ';' {
            let start = self.position.clone();
            let mut end = self.position.clone();
            end.advance(self.current_char);
            if [' ', '\n'].contains(&self.current_char) {
                self.advance();
            } else if self.current_char == '+' {
                tokens.push(Token::new(Tokens::Plus, start, end, None));
                self.advance();
            } else if self.current_char == '-' {
                tokens.push(Token::new(Tokens::Minus, start, end, None));
                self.advance();
            } else if self.current_char == '*' {
                tokens.push(Token::new(Tokens::Multiply, start, end, None));
                self.advance();
            } else if self.current_char == '/' {
                tokens.push(Token::new(Tokens::Divide, start, end, None));
                self.advance();
            } else if self.current_char == '(' {
                tokens.push(Token::new(Tokens::LeftParenthesis, start, end, None));
                self.advance();
            } else if self.current_char == ')' {
                tokens.push(Token::new(Tokens::RightParenthesis, start, end, None));
                self.advance();
            } else if self.current_char == '^' {
                tokens.push(Token::new(Tokens::Power, start, end, None));
                self.advance();
            } else if self.current_char == ':' {
                tokens.push(Token::new(Tokens::Colon, start, end, None));
                self.advance();
            } else if self.current_char == ',' {
                tokens.push(Token::new(Tokens::Comma, start, end, None));
                self.advance();
            } else if self.current_char == '"' {
                tokens.push(self.make_string());
            } else if self.current_char == '\'' {
                let result = self.make_char();
                if result.error.is_none() && !result.token.is_none() {
                    tokens.push(result.token.unwrap());
                } else {
                    return LexerResult::new(None, Some(result.error.unwrap()));
                };
            } else if self.current_char == '|' {
                let result = self.make_or();
                if result.error.is_none() && !result.token.is_none() {
                    tokens.push(result.token.unwrap());
                } else {
                    return LexerResult::new(None, Some(result.error.unwrap()));
                };
            } else if self.current_char == '&' {
                let result = self.make_and();
                if result.error.is_none() && !result.token.is_none() {
                    tokens.push(result.token.unwrap());
                } else {
                    return LexerResult::new(None, Some(result.error.unwrap()));
                };
            } else if self.current_char == '!' {
                tokens.push(self.make_not());
            } else if self.current_char == '<' {
                tokens.push(self.make_less_than());
            } else if self.current_char == '>' {
                tokens.push(self.make_greater_than());
            } else if self.current_char == '=' {
                tokens.push(self.make_equals());
            } else if get_number().contains(&self.current_char.to_digit(36).unwrap_or(69420))
                || self.current_char == '.'
            {
                tokens.push(self.make_number());
            } else if get_ascii_letters().contains(&self.current_char.to_string().as_str()) {
                tokens.push(self.make_identifiers());
            } else {
                let start = self.position.clone();
                let char = self.current_char.to_string();
                return LexerResult::new(
                    None,
                    Some(Error::new(
                        "Illegal Character",
                        start,
                        self.position.clone(),
                        format!("Unexpected Character '{}'.", char).as_str()
                    )),
                );
            }
        }
        tokens.push(Token::new(
            Tokens::EOF,
            self.position.clone(),
            self.position.clone(),
            None,
        ));
        LexerResult::new(Some(tokens), None)
    }

    fn make_number(&mut self) -> Token {
        let mut str_num = String::new();
        let mut dot_count = 0;
        let start = self.position.clone();

        while self.current_char != ';'
            && (get_number().contains(&self.current_char.to_digit(36).unwrap_or(69420))
                || self.current_char == '.')
        {
            if self.current_char == '.' {
                dot_count += 1;
                str_num.push('.');
                self.advance();
            } else {
                str_num.push(self.current_char);
                self.advance();
            }
        }

        self.advance();

        if dot_count != 0 {
            return Token::new(
                Tokens::Float,
                start,
                self.position.clone(),
                Some(DynType::Float(str_num.parse::<f32>().unwrap())),
            );
        }

        return Token::new(
            Tokens::Int,
            start,
            self.position.clone(),
            Some(DynType::Int(str_num.parse::<i64>().unwrap())),
        );
    }

    fn make_string(&mut self) -> Token {
        let mut str_raw = String::new();
        let start = self.position.clone();
        let mut escape = true;
        self.advance();

        while self.current_char != ';' && self.current_char != '"' || escape {
            if escape {
                if self.current_char == '\n' {
                    str_raw.push('\n');
                } else if self.current_char == '\t' {
                    str_raw.push('\t');
                } else {
                    str_raw.push(self.current_char);
                }
            } else {
                if self.current_char == '\\' {
                    escape = true;
                } else {
                    str_raw.push(self.current_char);
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
            Some(DynType::String(str_raw)),
        )
    }

    fn make_char(&mut self) -> LexerMethodResult {
        let start = self.position.clone();

        self.advance();
        let new_char = self.current_char;
        self.advance();

        if self.current_char != '\'' {
            return LexerMethodResult::new(
                None,
                Some(Error::new(
                    "Expected Charecter",
                    start,
                    self.position.clone(),
                    "Expected Charecter \"'\" because chars are unicode charecters."
                )),
            );
        }

        self.advance();

        LexerMethodResult::new(
            Some(Token::new(
                Tokens::Char,
                start,
                self.position.clone(),
                Some(DynType::Char(new_char)),
            )),
            None,
        )
    }

    fn make_equals(&mut self) -> Token {
        let start = self.position.clone();
        self.advance();

        if self.current_char == '=' {
            self.advance();
            return Token::new(Tokens::DoubleEquals, start, self.position.clone(), None);
        } else if self.current_char == '>' {
            self.advance();
            return Token::new(Tokens::Arrow, start, self.position.clone(), None);
        }

        self.advance();
        Token::new(Tokens::Equals, start, self.position.clone(), None)
    }

    fn make_less_than(&mut self) -> Token {
        let start = self.position.clone();
        self.advance();

        if self.current_char == '=' {
            return Token::new(Tokens::LessThanEquals, start, self.position.clone(), None);
        }

        self.advance();
        Token::new(Tokens::LessThan, start, self.position.clone(), None)
    }

    fn make_greater_than(&mut self) -> Token {
        let start = self.position.clone();
        self.advance();

        if self.current_char == '=' {
            return Token::new(
                Tokens::GreaterThanEquals,
                start,
                self.position.clone(),
                None,
            );
        }

        self.advance();
        Token::new(Tokens::GreaterThan, start, self.position.clone(), None)
    }

    fn make_not(&mut self) -> Token {
        let start = self.position.clone();
        self.advance();

        if self.current_char == '=' {
            self.advance();
            return Token::new(Tokens::NotEquals, start, self.position.clone(), None);
        }

        self.advance();
        Token::new(
            Tokens::Keyword,
            start,
            self.position.clone(),
            Some(DynType::String("not".to_string())),
        )
    }

    fn make_or(&mut self) -> LexerMethodResult {
        let start = self.position.clone();
        self.advance();

        if self.current_char == '|' {
            self.advance();
            return LexerMethodResult::new(
                Some(Token::new(
                    Tokens::Keyword,
                    start,
                    self.position.clone(),
                    Some(DynType::String("or".to_string())),
                )),
                None,
            );
        }

        self.advance();

        LexerMethodResult::new(
            None,
            Some(Error::new(
                "Expected Charecter",
                start,
                self.position.clone(),
                "Expected one more '|'"
            )),
        )
    }

    fn make_and(&mut self) -> LexerMethodResult {
        let start = self.position.clone();
        self.advance();

        if self.current_char == '&' {
            self.advance();
            return LexerMethodResult::new(
                Some(Token::new(
                    Tokens::Keyword,
                    start,
                    self.position.clone(),
                    Some(DynType::String("and".to_string())),
                )),
                None,
            );
        }

        self.advance();

        LexerMethodResult::new(
            None,
            Some(Error::new(
                "Expected Charecter",
                start,
                self.position.clone(),
                "Expected one more '|'"
            )),
        )
    }

    fn make_identifiers(&mut self) -> Token {
        let mut identifier = String::new();
        let start = self.position.clone();

        while self.current_char != ';'
            && get_ascii_letters_and_digits().contains(&self.current_char.to_string().as_str())
        {
            identifier.push(self.current_char);
            self.advance();
        }

        let identifier_type = if get_keywords().contains(&identifier) {
            Tokens::Keyword
        } else if identifier == "true".to_string() || identifier == "false".to_string() {
            Tokens::Boolean
        } else {
            Tokens::Identifier
        };
        Token::new(
            identifier_type,
            start,
            self.position.clone(),
            Some(if identifier_type != Tokens::Boolean {
                DynType::String(identifier)
            } else {
                DynType::Boolean(identifier.parse::<bool>().unwrap())
            }),
        )
    }
}
