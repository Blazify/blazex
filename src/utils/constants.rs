use crate::core::nodes::{
    binary_op_node::BinOpNode, boolean_node::BooleanNode, call_node::CallNode, char_node::CharNode,
    for_node::ForNode, fun_def::FunDef, if_node::IfNode, number_node::NumberNode,
    string_node::StringNode, unary_node::UnaryNode, var_access_node::VarAccessNode,
    var_assign_node::VarAssignNode, var_reassign_node::VarReassignNode, while_node::WhileNode,
};

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
    None,
}

pub fn get_keywords() -> Vec<String> {
    vec![
        string("val"),
        string("var"),
        string("and"),
        string("or"),
        string("not"),
        string("if"),
        string("then"),
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
}

#[derive(Debug, Clone)]
pub enum Nodes {
    Number(Box<NumberNode>),
    BinOp(Box<BinOpNode>),
    UnaryOp(Box<UnaryNode>),
    IfNode(Box<IfNode>),
    ForNode(Box<ForNode>),
    VarAssignNode(Box<VarAssignNode>),
    StringNode(Box<StringNode>),
    CharNode(Box<CharNode>),
    VarAccessNode(Box<VarAccessNode>),
    VarReassignNode(Box<VarReassignNode>),
    WhileNode(Box<WhileNode>),
    BooleanNode(Box<BooleanNode>),
    FunDef(Box<FunDef>),
    CallNode(Box<CallNode>),
}
