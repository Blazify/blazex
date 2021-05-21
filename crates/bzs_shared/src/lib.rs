/*
 * Copyright 2020 to 2021 BlazifyOrg
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *    http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
*/
#![allow(unused_must_use)]
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use codespan_reporting::term::{self};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Error as E, Formatter};

/*
* Enum of all the Token Types
*/
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

/*
* Enum of all the Token Values
*/
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
    /*
     * Convert a Token value to int if possible
     */
    pub fn into_int(&self) -> i128 {
        if let DynType::Int(i) = self {
            *i
        } else {
            panic!()
        }
    }

    /*
     * Convert a Token value to float if possible
     */
    pub fn into_float(&self) -> f64 {
        if let DynType::Float(i) = self {
            *i
        } else {
            panic!()
        }
    }

    /*
     * Convert a Token value to string if possible
     */
    pub fn into_string(&self) -> String {
        if let DynType::String(i) = self {
            i.to_string()
        } else {
            panic!()
        }
    }

    /*
     * Convert a Token value to charecter if possible
     */
    pub fn into_char(&self) -> char {
        if let DynType::Char(i) = self {
            *i
        } else {
            panic!()
        }
    }

    /*
     * Convert a Token value to boolean if possible
     */
    pub fn into_boolean(&self) -> bool {
        if let DynType::Boolean(i) = self {
            *i
        } else {
            panic!()
        }
    }
}

/*
* Raw Constant Enum returned by Bytecode Compiler
*/
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Constants {
    None,
    Null,
    Int(i128),
    Float(f64),
    String(String),
    Char(char),
    Boolean(bool),
    Function(Vec<u16>, ByteCode),
    RawArray(Vec<ByteCode>),
    RawObject(HashMap<usize, ByteCode>),
    RawClass(Option<(Vec<u16>, ByteCode)>, HashMap<usize, ByteCode>),
}

/*
* Custom Error struct for capturing errors
*/
#[derive(Debug, Clone)]
pub struct Error {
    pub name: &'static str,
    pub pos_start: Position,
    pub pos_end: Position,
    pub description: &'static str,
}

impl Error {
    /*
     * Creates a new Error Struct
     */
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

    /*
     * Prettifies the Error
     */
    pub fn prettify(&self) {
        let mut files = SimpleFiles::new();
        let file_id = files.add(self.pos_start.file_name, self.pos_start.file_content);

        let diagnostic = Diagnostic::error()
            .with_message(self.name)
            .with_labels(vec![Label::primary(
                file_id,
                (self.pos_start.index as usize)..(self.pos_end.index as usize),
            )
            .with_message(self.description)]);

        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();

        term::emit(&mut writer.lock(), &config, &files, &diagnostic);
    }
}

/*
* Position struct for error pretty-printing
*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub index: usize,
    pub file_name: &'static str,
    pub file_content: &'static str,
}

impl Position {
    /*
     * Creates a new Position Struct
     */
    pub fn new(index: usize, file_name: &'static str, file_content: &'static str) -> Position {
        Position {
            index,
            file_name,
            file_content,
        }
    }

    /*
     * Advances the position by one
     */
    pub fn advance(&mut self) -> Self {
        self.index += 1;
        self.clone()
    }
}

/*
* Token struct for tokens in a program
*/
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub r#type: Tokens,
    pub value: DynType,
    pub pos_start: Position,
    pub pos_end: Position,
}

impl Token {
    /*
     * Creates a new token
     */
    pub fn new(r#type: Tokens, pos_start: Position, pos_end: Position, value: DynType) -> Token {
        Token {
            r#type,
            value,
            pos_start,
            pos_end,
        }
    }

    /*
     * Matches a token based upon it's type and value
     */
    pub fn matches(&self, r#type: Tokens, value: DynType) -> bool {
        return self.r#type == r#type && self.value == value;
    }
}

/*
* Enum Node returned by Parser
*/
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
        constructor: Box<Option<(Vec<Token>, Node)>>,
        properties: Vec<(Token, Node)>,
        methods: Vec<(Token, Node)>,
    },
    ClassInitNode {
        name: Token,
        constructor_params: Vec<Node>,
    },
}

/*
* Bytecode Struct which contains the actual instructions and constants
*/
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ByteCode {
    pub instructions: Vec<u8>,
    pub constants: Vec<Constants>,
}

impl ByteCode {
    /*
     * Creates a new Bytecode instance
     */
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
