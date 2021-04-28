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
    pub current_char: Option<char>,
    pub position: Position,
}

impl Lexer {
    pub fn new(file_name: &'static str, text: &'static str) -> Lexer {
        let mut lexer = Lexer {
            file_name: String::from(file_name),
            text: String::from(text),
            current_char: None,
            position: Position::new(-1, 0, -1, file_name, text),
        };
        lexer.advance();
        lexer
    }

    fn advance(&mut self) {
        self.position.advance(self.current_char.unwrap_or(' '));
        if self.text.len() > self.position.index.try_into().unwrap() {
            let split: Vec<char> = self.text.chars().collect::<Vec<char>>();
            let index: i64 = self.position.index.try_into().unwrap();
            self.current_char = Some(split[index as usize]);
        } else {
            self.current_char = None;
        }
    }

    pub fn tokenize(&mut self) -> LexerResult {
        let mut tokens: Vec<Token> = vec![];

        while self.current_char.is_some() {
            let start = self.position.clone();
            let mut end = self.position.clone();
            end.advance(self.current_char.unwrap());

            if [' ', '\t'].contains(&self.current_char.unwrap()) {
                self.advance();
                continue;
            }

            if ['\n', ';'].contains(&self.current_char.unwrap()) {
                let pos_start = self.position.clone();
                self.advance();
                tokens.push(Token::new(
                    Tokens::Newline,
                    pos_start,
                    self.position.clone(),
                    DynType::None,
                ));
                continue;
            }

            let token = match self.current_char.unwrap() {
                '+' => Tokens::Plus,
                '-' => Tokens::Minus,
                '*' => Tokens::Multiply,
                '/' => Tokens::Divide,
                '(' => Tokens::LeftParenthesis,
                ')' => Tokens::RightParenthesis,
                '{' => Tokens::LeftCurlyBraces,
                '}' => Tokens::RightCurlyBraces,
                '[' => Tokens::LeftSquareBraces,
                ']' => Tokens::RightSquareBraces,
                '^' => Tokens::Power,
                ':' => Tokens::Colon,
                ',' => Tokens::Comma,
                '.' => Tokens::Dot,
                _ => Tokens::Unknown,
            };

            let mut token_is_unknown = false;
            if token == Tokens::Unknown {
                let mut res = LexerResult::new(None, None);
                match self.current_char.unwrap() {
                    '@' => self.skip_comment(),
                    '"' => tokens.push(self.make_string()),
                    '!' => tokens.push(self.make_not()),
                    '<' => tokens.push(self.make_less_than()),
                    '>' => tokens.push(self.make_greater_than()),
                    '=' => tokens.push(self.make_equals()),
                    '\'' => {
                        let result = self.make_char();
                        if result.error.is_none() && !result.token.is_none() {
                            tokens.push(result.token.unwrap());
                        } else {
                            res = LexerResult::new(None, Some(result.error.unwrap()));
                        };
                    }
                    '|' => {
                        let result = self.make_or();
                        if result.error.is_none() && !result.token.is_none() {
                            tokens.push(result.token.unwrap());
                        } else {
                            res = LexerResult::new(None, Some(result.error.unwrap()));
                        };
                    }
                    '&' => {
                        let result = self.make_and();
                        if result.error.is_none() && !result.token.is_none() {
                            tokens.push(result.token.unwrap());
                        } else {
                            res = LexerResult::new(None, Some(result.error.unwrap()));
                        };
                    }
                    _ => {
                        let no = self.current_char.unwrap().to_digit(36);
                        if no.is_some() {
                            if get_number().contains(&no.unwrap())
                                || self.current_char.unwrap() == '.'
                            {
                                tokens.push(self.make_number());
                            } else if get_ascii_letters()
                                .contains(&self.current_char.unwrap().to_string().as_str())
                            {
                                tokens.push(self.make_identifiers());
                            } else {
                                token_is_unknown = true;
                            }
                        } else {
                            token_is_unknown = true;
                        }
                    }
                }

                if res.error.is_some() {
                    return res;
                }
            } else {
                tokens.push(Token::new(token, start, end, DynType::None));
                self.advance();
            }

            if token_is_unknown {
                let start_1 = self.position.clone();
                let char = self.current_char.unwrap().to_string();
                return LexerResult::new(
                    None,
                    Some(Error::new(
                        "Illegal Character",
                        start_1,
                        self.position.clone(),
                        Box::leak(
                            format!("Unexpected Character '{}'", char)
                                .to_owned()
                                .into_boxed_str(),
                        ),
                    )),
                );
            }
        }

        tokens.push(Token::new(
            Tokens::EOF,
            self.position.clone(),
            self.position.clone(),
            DynType::None,
        ));
        LexerResult::new(Some(tokens), None)
    }

    fn make_number(&mut self) -> Token {
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
                DynType::Float(str_num.parse::<f32>().unwrap()),
            )
        } else {
            Token::new(
                Tokens::Int,
                start,
                self.position.clone(),
                DynType::Int(str_num.parse::<i64>().unwrap()),
            )
        };
    }

    fn make_string(&mut self) -> Token {
        let mut str_raw = String::new();
        let start = self.position.clone();
        let mut escape = true;
        self.advance();

        while self.current_char.is_some() || escape {
            if self.current_char.unwrap() == '"' {
                break;
            }
            if escape {
                if self.current_char.unwrap_or(' ') == '\n' {
                    str_raw.push('\n');
                } else if self.current_char.unwrap_or(' ') == '\t' {
                    str_raw.push('\t');
                } else {
                    str_raw.push(self.current_char.unwrap());
                }
            } else {
                if self.current_char.unwrap_or(' ') == '\\' {
                    escape = true;
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

    fn make_char(&mut self) -> LexerMethodResult {
        let start = self.position.clone();

        self.advance();
        let new_char = self.current_char;
        self.advance();

        if self.current_char.unwrap_or(' ') != '\'' {
            return LexerMethodResult::new(
                None,
                Some(Error::new(
                    "Expected Character",
                    start,
                    self.position.clone(),
                    "Expected Character \"'\" because chars are unicode characters.",
                )),
            );
        }

        self.advance();

        LexerMethodResult::new(
            Some(Token::new(
                Tokens::Char,
                start,
                self.position.clone(),
                DynType::Char(new_char.unwrap()),
            )),
            None,
        )
    }

    fn make_equals(&mut self) -> Token {
        let start = self.position.clone();
        self.advance();

        if self.current_char.unwrap_or(' ') == '=' {
            self.advance();
            return Token::new(
                Tokens::DoubleEquals,
                start,
                self.position.clone(),
                DynType::None,
            );
        } else if self.current_char.unwrap_or(' ') == '>' {
            self.advance();
            return Token::new(Tokens::Arrow, start, self.position.clone(), DynType::None);
        }

        Token::new(Tokens::Equals, start, self.position.clone(), DynType::None)
    }

    fn make_less_than(&mut self) -> Token {
        let start = self.position.clone();
        self.advance();

        if self.current_char.unwrap_or(' ') == '=' {
            return Token::new(
                Tokens::LessThanEquals,
                start,
                self.position.clone(),
                DynType::None,
            );
        }

        Token::new(
            Tokens::LessThan,
            start,
            self.position.clone(),
            DynType::None,
        )
    }

    fn make_greater_than(&mut self) -> Token {
        let start = self.position.clone();
        self.advance();

        if self.current_char.unwrap_or(' ') == '=' {
            return Token::new(
                Tokens::GreaterThanEquals,
                start,
                self.position.clone(),
                DynType::None,
            );
        }

        Token::new(
            Tokens::GreaterThan,
            start,
            self.position.clone(),
            DynType::None,
        )
    }

    fn make_not(&mut self) -> Token {
        let start = self.position.clone();
        self.advance();

        if self.current_char.unwrap_or(' ') == '=' {
            self.advance();
            return Token::new(
                Tokens::NotEquals,
                start,
                self.position.clone(),
                DynType::None,
            );
        }

        Token::new(
            Tokens::Keyword,
            start,
            self.position.clone(),
            DynType::String("not".to_string()),
        )
    }

    fn make_or(&mut self) -> LexerMethodResult {
        let start = self.position.clone();
        self.advance();

        if self.current_char.unwrap_or(' ') == '|' {
            self.advance();
            return LexerMethodResult::new(
                Some(Token::new(
                    Tokens::Keyword,
                    start,
                    self.position.clone(),
                    DynType::String("or".to_string()),
                )),
                None,
            );
        }

        LexerMethodResult::new(
            None,
            Some(Error::new(
                "Expected Character",
                start,
                self.position.clone(),
                "Expected one more '|'",
            )),
        )
    }

    fn make_and(&mut self) -> LexerMethodResult {
        let start = self.position.clone();
        self.advance();

        if self.current_char.unwrap_or(' ') == '&' {
            self.advance();
            return LexerMethodResult::new(
                Some(Token::new(
                    Tokens::Keyword,
                    start,
                    self.position.clone(),
                    DynType::String("and".to_string()),
                )),
                None,
            );
        }

        LexerMethodResult::new(
            None,
            Some(Error::new(
                "Expected Character",
                start,
                self.position.clone(),
                "Expected one more '&'",
            )),
        )
    }

    fn make_identifiers(&mut self) -> Token {
        let mut identifier = String::new();
        let start = self.position.clone();

        while self.current_char.is_some() {
            if !get_ascii_letters_and_digits()
                .contains(&self.current_char.unwrap().to_string().as_str())
            {
                break;
            }
            identifier.push(self.current_char.unwrap());
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
            if identifier_type != Tokens::Boolean {
                DynType::String(identifier)
            } else {
                DynType::Boolean(identifier.parse::<bool>().unwrap())
            },
        )
    }

    pub fn skip_comment(&mut self) {
        self.advance();

        while self.current_char.unwrap_or(' ') != '\n' {
            self.advance();
        }

        self.advance();
    }
}
