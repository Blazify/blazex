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
use inkwell::context::Context;
use inkwell::types::{AnyTypeEnum, BasicTypeEnum, FunctionType};
use inkwell::AddressSpace;

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
    PlusEquals,
    MinusEquals,
    MultiplyEquals,
    DivideEquals,
    PowerEquals,
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

pub fn to_static_str(string: String) -> &'static str {
    Box::leak(string.to_owned().into_boxed_str())
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
        return if let DynType::Float(i) = self {
            *i
        } else if let DynType::Int(i) = self {
            *i as f64
        } else {
            panic!()
        };
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
    pub typee: Tokens,
    pub value: DynType,
    pub pos_start: Position,
    pub pos_end: Position,
}

impl Token {
    /*
     * Creates a new token
     */
    pub fn new(typee: Tokens, pos_start: Position, pos_end: Position, value: DynType) -> Token {
        Token {
            typee,
            value,
            pos_start,
            pos_end,
        }
    }

    /*
     * Matches a token based upon it's type and value
     */
    pub fn matches(&self, typee: Tokens, value: DynType) -> bool {
        return self.typee == typee && self.value == value;
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
        typee: Token,
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
        arg_tokens: Vec<(Token, Type)>,
        return_type: Type,
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
    BinaryNode {
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
        methods: Vec<(Token, Vec<Token>, Node)>,
    },
    ClassInitNode {
        name: Token,
        constructor_params: Vec<Node>,
    },
    ExternNode {
        name: Token,
        arg_tokens: Vec<Type>,
        return_type: Type,
        var_args: bool,
    },
}

impl Node {
    pub fn get_pos(&self) -> (Position, Position) {
        match self {
            Node::WhileNode {
                condition_node,
                body_node,
            } => (condition_node.get_pos().0, body_node.get_pos().1),
            Node::VarReassignNode {
                name,
                typee: _,
                value,
            } => (name.pos_start, value.get_pos().1),
            Node::VarAssignNode {
                name,
                value,
                reassignable: _,
            } => (name.pos_start, value.get_pos().1),
            Node::VarAccessNode { token } => (token.pos_start, token.pos_end),
            Node::UnaryNode { node, op_token } => (node.get_pos().0, op_token.pos_end),
            Node::StringNode { token } => (token.pos_start, token.pos_end),
            Node::NumberNode { token } => (token.pos_start, token.pos_end),
            Node::IfNode { cases, else_case } => (
                cases.first().unwrap().0.get_pos().0,
                if else_case.is_some() {
                    else_case.clone().unwrap().get_pos().1
                } else {
                    cases.last().unwrap().1.get_pos().1
                },
            ),
            Node::FunDef {
                name,
                body_node,
                arg_tokens,
                return_type: _,
            } => (
                if name.is_some() {
                    name.clone().unwrap().pos_start
                } else if !arg_tokens.is_empty() {
                    arg_tokens.first().unwrap().0.pos_start
                } else {
                    body_node.get_pos().0
                },
                body_node.get_pos().1,
            ),
            Node::ForNode {
                var_name_token,
                start_value: _,
                end_value: _,
                body_node,
                step_value_node: _,
            } => (var_name_token.pos_start, body_node.get_pos().1),
            Node::CharNode { token } => (token.pos_start, token.pos_end),
            Node::CallNode { node_to_call, args } => (
                node_to_call.get_pos().0,
                if !args.is_empty() {
                    args.last().unwrap().get_pos().1
                } else {
                    node_to_call.get_pos().1
                },
            ),
            Node::BooleanNode { token } => (token.pos_start, token.pos_end),
            Node::BinaryNode {
                left,
                right,
                op_token: _,
            } => (left.get_pos().0, right.get_pos().1),
            Node::ArrayNode { element_nodes } => (
                element_nodes.first().unwrap().get_pos().0,
                element_nodes.last().unwrap().get_pos().1,
            ),
            Node::ArrayAcess { array, index } => (array.get_pos().0, index.get_pos().1),
            Node::Statements { statements } => (
                statements.first().unwrap().get_pos().0,
                statements.last().unwrap().get_pos().1,
            ),
            Node::ReturnNode { value } => (
                value.clone().unwrap().get_pos().0,
                value.clone().unwrap().get_pos().1,
            ),
            Node::ObjectDefNode { properties } => (
                properties.first().unwrap().0.pos_start,
                properties.last().unwrap().1.get_pos().1,
            ),
            Node::ObjectPropAccess { object, property } => (object.get_pos().0, property.pos_end),
            Node::ObjectPropEdit {
                object,
                property: _,
                new_val,
            } => (object.get_pos().0, new_val.get_pos().1),
            Node::ClassDefNode {
                name,
                constructor: _,
                properties: _,
                methods,
            } => (name.pos_start, methods.last().unwrap().2.get_pos().1),
            Node::ClassInitNode {
                name,
                constructor_params,
            } => (
                name.pos_start,
                if !constructor_params.is_empty() {
                    constructor_params.last().unwrap().get_pos().1
                } else {
                    name.pos_end
                },
            ),
            Node::ExternNode { name, .. } => (name.pos_start, name.pos_end),
        }
    }
}

pub fn try_any_to_basic(k: AnyTypeEnum) -> BasicTypeEnum {
    match k {
        AnyTypeEnum::ArrayType(x) => x.into(),
        AnyTypeEnum::FloatType(x) => x.into(),
        AnyTypeEnum::FunctionType(x) => x.ptr_type(AddressSpace::Generic).into(),
        AnyTypeEnum::IntType(x) => x.into(),
        AnyTypeEnum::PointerType(x) => x.into(),
        AnyTypeEnum::StructType(x) => x.into(),
        AnyTypeEnum::VectorType(x) => x.into(),
        AnyTypeEnum::VoidType(_) => panic!("void not convertible to basic type"),
    }
}

pub fn any_fn_type<'ctx>(
    ret_type: AnyTypeEnum<'ctx>,
    args_types: &[BasicTypeEnum<'ctx>],
    var_args: bool,
) -> FunctionType<'ctx> {
    match ret_type {
        AnyTypeEnum::ArrayType(x) => x.fn_type(args_types, var_args),
        AnyTypeEnum::FloatType(x) => x.fn_type(args_types, var_args),
        AnyTypeEnum::FunctionType(x) => x
            .ptr_type(AddressSpace::Generic)
            .fn_type(args_types, var_args),
        AnyTypeEnum::IntType(x) => x.fn_type(args_types, var_args),
        AnyTypeEnum::PointerType(x) => x.fn_type(args_types, var_args),
        AnyTypeEnum::StructType(x) => x.fn_type(args_types, var_args),
        AnyTypeEnum::VectorType(x) => x.fn_type(args_types, var_args),
        AnyTypeEnum::VoidType(x) => x.fn_type(args_types, var_args),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Boolean,
    Char,
    String,
    Void,
    Function(Vec<Type>, Box<Type>),
    Custom(&'static str),
}

impl<'ctx> Type {
    pub fn to_llvm_type(&self, ctx: &'ctx Context) -> AnyTypeEnum<'ctx> {
        match self {
            Type::Int => AnyTypeEnum::IntType(ctx.i128_type()),
            Type::Float => AnyTypeEnum::FloatType(ctx.f64_type()),
            Type::Boolean => AnyTypeEnum::IntType(ctx.bool_type()),
            Type::Char => AnyTypeEnum::IntType(ctx.i8_type()),
            Type::String => AnyTypeEnum::PointerType(ctx.i8_type().ptr_type(AddressSpace::Generic)),
            Type::Void => AnyTypeEnum::VoidType(ctx.void_type()),
            Type::Function(params, ret) => any_fn_type(
                ret.to_llvm_type(ctx),
                &params
                    .iter()
                    .map(|x| try_any_to_basic(x.to_llvm_type(ctx)))
                    .collect::<Vec<BasicTypeEnum>>()[..],
                false,
            )
            .into(),
            Type::Custom(_) => panic!("Custom types aren't supported yet!"),
        }
    }
}
