#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Tokens {
    Int,
    Float,
    String,
    Boolean,
    Char,
    Colon,
    Comma,
    Arrow,
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParenthesis,
    RightParenthesis,
    LeftCurlyBraces,
    RightCurlyBraces,
    Power,
    Keyword,
    Identifier,
    Equals,
    DoubleEquals,
    NotEquals,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,
    EOF,
    Unknown,
}

pub fn get_keywords() -> Vec<String> {
    vec![
        string("val"),
        string("var"),
        string("and"),
        string("or"),
        string("not"),
        string("if"),
        string("else"),
        string("for"),
        string("to"),
        string("step"),
        string("while"),
        string("fun"),
    ]
}

pub fn get_number() -> Vec<u32> {
    vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
}

fn string(str: &str) -> String {
    return String::from(str);
}

pub fn get_ascii_letters() -> Vec<&'static str> {
    "_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
        .split("")
        .collect::<Vec<&str>>()
}

pub fn get_ascii_letters_and_digits() -> Vec<&'static str> {
    "_0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
        .split("")
        .collect::<Vec<&str>>()
}

#[derive(Debug, PartialEq, Clone)]
pub enum DynType {
    Int(i64),
    Float(f32),
    String(String),
    Char(char),
    Boolean(bool),
    None,
}
