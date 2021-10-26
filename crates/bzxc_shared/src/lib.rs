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
use bzxc_llvm_wrapper::context::Context;
use bzxc_llvm_wrapper::types::{AnyTypeEnum, BasicTypeEnum};
use bzxc_llvm_wrapper::AddressSpace;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

/*
* Enum of all the Token Types
*/
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tokens {
    Int(i128),
    Float(f64),
    String(&'static str),
    Boolean(bool),
    Char(char),
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
    Keyword(&'static str),
    Identifier(&'static str),
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

impl Tokens {
    /*
     * Convert a Token value to int if possible
     */
    pub fn into_int(&self) -> i128 {
        if let Tokens::Int(i) = self {
            *i
        } else {
            panic!()
        }
    }

    /*
     * Convert a Token value to float if possible
     */
    pub fn into_float(&self) -> f64 {
        return if let Tokens::Float(i) = self {
            *i
        } else if let Tokens::Int(i) = self {
            *i as f64
        } else {
            panic!()
        };
    }

    /*
     * Convert a Token value to string if possible
     */
    pub fn into_string(&self) -> String {
        if let Tokens::String(i) | Tokens::Identifier(i) | Tokens::Keyword(i) = self {
            i.to_string()
        } else {
            panic!()
        }
    }

    /*
     * Convert a Token value to charecter if possible
     */
    pub fn into_char(&self) -> char {
        if let Tokens::Char(i) = self {
            *i
        } else {
            panic!()
        }
    }

    /*
     * Convert a Token value to boolean if possible
     */
    pub fn into_boolean(&self) -> bool {
        if let Tokens::Boolean(i) = self {
            *i
        } else {
            panic!()
        }
    }
}

/*
* Custom Error struct for capturing errors
*/
#[derive(Debug, Clone, Copy)]
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
    /* Just for Testing */
    pub fn proto() -> Self {
        Position {
            file_content: "",
            file_name: "",
            index: 0,
        }
    }

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
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Token {
    pub value: Tokens,
    pub pos_start: Position,
    pub pos_end: Position,
}

impl Token {
    /*
     * Creates a new token
     */
    pub fn new(value: Tokens, pos_start: Position, pos_end: Position) -> Token {
        Token {
            value,
            pos_start,
            pos_end,
        }
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
        // TODO: arg_tokens: Vec<Token>
        arg_tokens: Vec<(Token, Type)>,
        // TODO: remove return_type
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
    ObjectMethodCall {
        object: Box<Node>,
        property: Token,
        args: Vec<Node>,
    },
    ClassDefNode {
        // TODO: methods: Vec<(Token, Vec<Token>, Node)>,
        methods: Vec<(Token, Vec<(Token, Type)>, Node, Type)>,
        properties: Vec<(Token, Node)>,
        // TODO: (Vec<Token>, Box<Node>)
        constructor: (Vec<(Token, Type)>, Box<Node>),
        name: Token,
    },
    ClassInitNode {
        name: Token,
        constructor_params: Vec<Node>,
    },
    ExternNode {
        name: Token,
        // TODO: arg_tokens: Vec<String>,
        arg_tokens: Vec<Type>,
        // TODO: return_type: String,
        return_type: Type,
        // TODO: remove var_args
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
            Node::ArrayNode { element_nodes } => {
                if !element_nodes.is_empty() {
                    (
                        element_nodes.first().unwrap().get_pos().0,
                        element_nodes.last().unwrap().get_pos().1,
                    )
                } else {
                    (Position::proto(), Position::proto())
                }
            }
            Node::ArrayAcess { array, index } => (array.get_pos().0, index.get_pos().1),
            Node::Statements { statements } => (
                statements.first().unwrap().get_pos().0,
                statements.last().unwrap().get_pos().1,
            ),
            Node::ReturnNode { value } => {
                if let Some(val) = *value.clone() {
                    (val.get_pos().0, val.get_pos().1)
                } else {
                    (Position::proto(), Position::proto())
                }
            }
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
            Node::ObjectMethodCall {
                object,
                property,
                args,
            } => (
                object.get_pos().0,
                if !args.is_empty() {
                    args.last().unwrap().get_pos().1
                } else {
                    property.pos_end
                },
            ),
            Node::ClassDefNode { name, .. } => (name.pos_start, name.pos_end),
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TypedNode<'ctx> {
    Statements {
        statements: &'ctx [&'ctx TypedNode<'ctx>],
    },
    While {
        condition_node: &'ctx TypedNode<'ctx>,
        body_node: &'ctx TypedNode<'ctx>,
    },
    VarReassign {
        name: &'static str,
        typee: Token,
        value: &'ctx TypedNode<'ctx>,
    },
    VarAssign {
        name: &'static str,
        value: &'ctx TypedNode<'ctx>,
    },
    VarAccess {
        token: &'static str,
    },
    Unary {
        node: &'ctx TypedNode<'ctx>,
        op_token: Token,
    },
    String {
        token: &'static str,
    },
    Int {
        token: i128,
    },
    Float {
        token: f64,
    },
    If {
        cases: &'ctx [(&'ctx TypedNode<'ctx>, &'ctx TypedNode<'ctx>)],
        else_case: Option<&'ctx TypedNode<'ctx>>,
    },
    Fun {
        name: Option<&'static str>,
        arg_tokens: &'ctx [(&'static str, AnyTypeEnum<'ctx>)],
        body: &'ctx TypedNode<'ctx>,
        return_type: AnyTypeEnum<'ctx>,
    },
    For {
        var_name_token: &'static str,
        start_value: &'ctx TypedNode<'ctx>,
        end_value: &'ctx TypedNode<'ctx>,
        body_node: &'ctx TypedNode<'ctx>,
        step_value_node: &'ctx TypedNode<'ctx>,
    },
    Char {
        token: char,
    },
    Call {
        node_to_call: &'ctx TypedNode<'ctx>,
        args: &'ctx [&'ctx TypedNode<'ctx>],
    },
    Boolean {
        token: bool,
    },
    Binary {
        left: &'ctx TypedNode<'ctx>,
        right: &'ctx TypedNode<'ctx>,
        op_token: Token,
    },
    Array {
        typee: &'ctx AnyTypeEnum<'ctx>,
        element_nodes: &'ctx [&'ctx TypedNode<'ctx>],
    },
    Index {
        array: &'ctx TypedNode<'ctx>,
        index: &'ctx TypedNode<'ctx>,
    },
    Return {
        value: Option<&'ctx TypedNode<'ctx>>,
    },
    Object {
        properties: &'ctx [(&'static str, &'ctx TypedNode<'ctx>)],
    },
    ObjectAccess {
        object: &'ctx TypedNode<'ctx>,
        property: &'static str,
    },
    ObjectEdit {
        object: &'ctx TypedNode<'ctx>,
        property: &'static str,
        new_val: &'ctx TypedNode<'ctx>,
    },
    ObjectCall {
        object: &'ctx TypedNode<'ctx>,
        property: &'static str,
        args: &'ctx [&'ctx TypedNode<'ctx>],
    },
    Class {
        name: &'static str,
        constructor: (
            &'ctx [(&'static str, AnyTypeEnum<'ctx>)],
            &'ctx TypedNode<'ctx>,
        ),
        properties: &'ctx [(&'static str, &'ctx TypedNode<'ctx>)],
        methods: &'ctx [(
            &'static str,
            &'ctx [(&'static str, AnyTypeEnum<'ctx>)],
            &'ctx TypedNode<'ctx>,
            AnyTypeEnum<'ctx>,
        )],
    },
    ClassInit {
        name: &'static str,
        constructor_params: &'ctx [&'ctx TypedNode<'ctx>],
    },
    Extern {
        name: &'static str,
        arg_tokens: &'ctx [AnyTypeEnum<'ctx>],
        return_type: AnyTypeEnum<'ctx>,
        var_args: bool,
    },
}

// TODO: remove `Type` Struct and impl
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Boolean,
    Char,
    String,
    Void,
    Function(Vec<Type>, Box<Type>),
    Array(Box<Type>, Token),
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
            Type::Void => ctx
                .struct_type(&[], false)
                .ptr_type(AddressSpace::Generic)
                .into(),
            Type::Function(params, ret) => ret
                .to_llvm_type(ctx)
                .fn_type(
                    &params
                        .iter()
                        .map(|x| x.to_llvm_type(ctx).to_basic_type_enum())
                        .collect::<Vec<BasicTypeEnum>>()[..],
                    false,
                )
                .into(),
            Type::Array(typee, size) => {
                let size = size.value.into_int() as u32;
                typee.to_llvm_type(ctx).array_type(size).into()
            }
            Type::Custom(_) => panic!("Custom types aren't supported yet!"),
        }
    }
}
