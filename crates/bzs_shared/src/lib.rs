/*
   Copyright 2021 BlazifyOrg
   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at
       http://www.apache.org/licenses/LICENSE-2.0
   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Error as E, Formatter};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tokens {
    Int,
    Float,
    String,
    Boolean,
    Char,
    Colon,
    Comma,
    Dot,
    Arrow,
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParenthesis,
    RightParenthesis,
    LeftCurlyBraces,
    RightCurlyBraces,
    LeftSquareBraces,
    RightSquareBraces,
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
    Newline,
    EOF,
    Unknown,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DynType {
    Int(i128),
    Float(f64),
    String(String),
    Char(char),
    Boolean(bool),
    None,
}

impl DynType {
    pub fn into_int(&self) -> i128 {
        if let DynType::Int(i) = self {
            *i
        } else {
            panic!()
        }
    }

    pub fn into_float(&self) -> f64 {
        if let DynType::Float(i) = self {
            *i
        } else {
            panic!()
        }
    }

    pub fn into_string(&self) -> String {
        if let DynType::String(i) = self {
            i.to_string()
        } else {
            panic!()
        }
    }

    pub fn into_char(&self) -> char {
        if let DynType::Char(i) = self {
            *i
        } else {
            panic!()
        }
    }

    pub fn into_boolean(&self) -> bool {
        if let DynType::Boolean(i) = self {
            *i
        } else {
            panic!()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Constants {
    None,
    Int(i128),
    Float(f64),
    String(String),
    Char(char),
    Boolean(bool),
    Function(Vec<u16>, ByteCode),
    RawArray(Vec<ByteCode>),
    RawObject(HashMap<usize, ByteCode>),
    Array(Vec<Constants>),
    Object(HashMap<usize, Constants>),
}

impl Constants {
    pub fn property_edit(&mut self, i: usize, val: Constants) {
        match self {
            Self::Object(map) => {
                map.insert(i, val);
            }
            _ => panic!("property_edit called on unexpected type"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    pub name: &'static str,
    pub pos_start: Position,
    pub pos_end: Position,
    pub description: &'static str,
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
        }
    }

    pub fn prettify(&self) -> String {
        format!(
            "\u{001b}[31;1m{}: {}\nFile {}, line {}\n\n {}\u{001b}[0m",
            self.name,
            self.description,
            self.pos_start.file_name,
            self.pos_start.line + 1,
            self.string_with_arrows(),
        )
    }

    fn string_with_arrows(&self) -> String {
        let mut res = String::new();
        let text = self.pos_start.file_content.to_string().clone();

        let mut idx_start = std::cmp::max(
            text[0..self.pos_start.index as usize]
                .rfind("\n")
                .unwrap_or(0),
            0,
        );
        let mut idx_end = text[(idx_start + 1)..(text.len() - 1)]
            .find("\n")
            .unwrap_or(0);
        if idx_end < 0 as usize {
            idx_end = text.len();
        }
        let line_count = self.pos_end.line - self.pos_start.line + 1;

        for i in 0..line_count {
            let line = &text[idx_start..(idx_end + idx_start)];

            let mut col_start = 0;
            if i == 0 {
                col_start = self.pos_start.column;
            }

            let mut col_end = line.len() as i128 - 1;
            if i == (line_count - 1) {
                col_end = self.pos_end.column;
            }

            res.push_str(line);
            res.push('\n');
            res = format!(
                "{}{}",
                res,
                " ".repeat((col_start) as usize) + &*"^".repeat((col_end - col_start) as usize)
            );

            idx_start = idx_end;
            idx_end = text[(idx_start + 1)..(text.len() - 1)]
                .find("\n")
                .unwrap_or(0);
            if idx_end < 0 as usize {
                idx_end = text.len();
            }
        }

        res.replacen("\t", "", res.len())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub index: i128,
    pub line: i128,
    pub column: i128,
    pub file_name: &'static str,
    pub file_content: &'static str,
}

impl Position {
    pub fn new(
        index: i128,
        line: i128,
        column: i128,
        file_name: &'static str,
        file_content: &'static str,
    ) -> Position {
        Position {
            index,
            line,
            column,
            file_name,
            file_content,
        }
    }

    pub fn advance(&mut self, character: char) -> Self {
        self.index += 1;
        self.column += 1;
        if character == '\n' {
            self.line += 1;
            self.column += 1;
        }
        self.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub r#type: Tokens,
    pub value: DynType,
    pub pos_start: Position,
    pub pos_end: Position,
}

impl Token {
    pub fn new(r#type: Tokens, pos_start: Position, pos_end: Position, value: DynType) -> Token {
        Token {
            r#type,
            value,
            pos_start,
            pos_end,
        }
    }

    pub fn matches(&self, r#type: Tokens, value: DynType) -> bool {
        return self.r#type == r#type && self.value == value;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    WhileNode {
        condition_node: Box<Node>,
        body_node: Box<Node>,
    },
    VarReassignNode {
        name: Token,
        value: Box<Node>,
    },
    VarAssignNode {
        name: Token,
        value: Box<Node>,
        reassignable: bool,
    },
    VarAccessNode {
        token: Token,
    },
    UnaryNode {
        node: Box<Node>,
        op_token: Token,
    },
    StringNode {
        token: Token,
    },
    NumberNode {
        token: Token,
    },
    IfNode {
        cases: Vec<(Node, Node)>,
        else_case: Box<Option<Node>>,
    },
    FunDef {
        name: Option<Token>,
        body_node: Box<Node>,
        arg_tokens: Vec<Token>,
    },
    ForNode {
        var_name_token: Token,
        start_value: Box<Node>,
        end_value: Box<Node>,
        body_node: Box<Node>,
        step_value_node: Box<Node>,
    },
    CharNode {
        token: Token,
    },
    CallNode {
        node_to_call: Box<Node>,
        args: Vec<Node>,
    },
    BooleanNode {
        token: Token,
    },
    BinOpNode {
        left: Box<Node>,
        right: Box<Node>,
        op_token: Token,
    },
    ArrayNode {
        element_nodes: Vec<Node>,
    },
    ArrayAcess {
        array: Box<Node>,
        index: Box<Node>,
    },
    Statements {
        statements: Vec<Node>,
    },
    ReturnNode {
        value: Box<Option<Node>>,
    },
    ObjectDefNode {
        properties: Vec<(Token, Node)>,
    },
    ObjectPropAccess {
        object: Box<Node>,
        property: Token,
    },
    ObjectPropEdit {
        object: Box<Node>,
        property: Token,
        new_val: Box<Node>,
    },
    ClassDefNode {
        name: Token,
        constructor: Box<Option<Node>>,
        properties: Vec<(Token, Node)>,
        methods: Vec<(Token, Node)>,
    },
    ClassInitNode {
        name: Token,
        constructor_params: Vec<Node>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByteCode {
    pub instructions: Vec<u8>,
    pub constants: Vec<Constants>,
}

impl ByteCode {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
        }
    }
}

impl Display for ByteCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), E> {
        write!(
            f,
            "\nInstructions: {}\nConstants: {}\n",
            self.instructions
                .iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<String>>()
                .join(" "),
            self.constants
                .iter()
                .map(|x| format!("{:?}", x))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
