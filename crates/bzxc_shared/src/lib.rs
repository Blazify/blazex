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
use std::collections::BTreeMap;
use std::hash::Hash;

use bzxc_llvm_wrapper::context::Context;
use bzxc_llvm_wrapper::types::BasicTypeEnum;
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
    pub fn into_int(self) -> i128 {
        if let Tokens::Int(i) = self {
            i
        } else {
            panic!()
        }
    }

    /*
     * Convert a Token value to float if possible
     */
    pub fn into_float(self) -> f64 {
        return if let Tokens::Float(i) = self {
            i
        } else if let Tokens::Int(i) = self {
            i as f64
        } else {
            panic!()
        };
    }

    /*
     * Convert a Token value to string if possible
     */
    pub fn into_string(self) -> String {
        if let Tokens::String(i) | Tokens::Identifier(i) | Tokens::Keyword(i) = self {
            i.to_string()
        } else {
            panic!()
        }
    }

    /*
     * Convert a Token value to charecter if possible
     */
    pub fn into_char(self) -> char {
        if let Tokens::Char(i) = self {
            i
        } else {
            panic!()
        }
    }

    /*
     * Convert a Token value to boolean if possible
     */
    pub fn into_boolean(self) -> bool {
        if let Tokens::Boolean(i) = self {
            i
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
        methods: Vec<(Token, Vec<Token>, Node)>,
        properties: Vec<(Token, Node)>,
        constructor: (Vec<Token>, Box<Node>),
        name: Token,
    },
    ClassInitNode {
        name: Token,
        constructor_params: Vec<Node>,
    },
    ExternNode {
        name: Token,
        arg_tokens: Vec<String>,
        return_type: String,
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
            } => (
                if name.is_some() {
                    name.clone().unwrap().pos_start
                } else if !arg_tokens.is_empty() {
                    arg_tokens.first().unwrap().pos_start
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

#[derive(Debug, Clone)]
pub enum LLVMNode<'ctx> {
    Statements(Vec<Self>),

    Int {
        ty: BasicTypeEnum<'ctx>,
        val: i128,
    },
    Float {
        ty: BasicTypeEnum<'ctx>,
        val: f64,
    },
    Boolean {
        ty: BasicTypeEnum<'ctx>,
        val: bool,
    },
    Char {
        ty: BasicTypeEnum<'ctx>,
        val: char,
    },
    String {
        ty: BasicTypeEnum<'ctx>,
        val: String,
    },
    Unary {
        ty: BasicTypeEnum<'ctx>,
        val: Box<Self>,
        op_token: Token,
    },
    Binary {
        ty: BasicTypeEnum<'ctx>,
        left: Box<Self>,
        right: Box<Self>,
        op_token: Token,
    },
    Fun {
        name: String,
        ty: BasicTypeEnum<'ctx>,
        params: Vec<(String, BasicTypeEnum<'ctx>)>,
        body: Box<Self>,
    },
    Let {
        name: String,
        ty: BasicTypeEnum<'ctx>,
        val: Box<Self>,
    },
    Var {
        ty: BasicTypeEnum<'ctx>,
        name: String,
    },
    Call {
        ty: BasicTypeEnum<'ctx>,
        fun: Box<Self>,
        args: Vec<Self>,
    },
    Return {
        ty: BasicTypeEnum<'ctx>,
        val: Box<Self>,
    },
    Null {
        ty: BasicTypeEnum<'ctx>,
    },
    If {
        ty: BasicTypeEnum<'ctx>,
        cases: Vec<(Self, Self)>,
        else_case: Option<Box<Self>>,
    },
    While {
        ty: BasicTypeEnum<'ctx>,
        cond: Box<Self>,
        body: Box<Self>,
    },
    For {
        ty: BasicTypeEnum<'ctx>,
        var: String,
        start: Box<Self>,
        end: Box<Self>,
        step: Box<Self>,
        body: Box<Self>,
    },
    Array {
        ty: BasicTypeEnum<'ctx>,
        elements: Vec<Self>,
    },
    Index {
        ty: BasicTypeEnum<'ctx>,
        array: Box<Self>,
        idx: Box<Self>,
    },
    Object {
        ty: BasicTypeEnum<'ctx>,
        properties: Vec<(String, Self)>,
    },
    ObjectAccess {
        ty: BasicTypeEnum<'ctx>,
        object: Box<Self>,
        property: String,
    },
    ObjectEdit {
        ty: BasicTypeEnum<'ctx>,
        object: Box<Self>,
        property: String,
        new_val: Box<Self>,
    },
    ObjectMethodCall {
        ty: BasicTypeEnum<'ctx>,
        object: Box<Self>,
        property: String,
        args: Vec<Self>,
    },
    Class {
        ty: BasicTypeEnum<'ctx>,
        name: String,
        properties: Vec<(String, Self)>,
        methods: Vec<(String, Self)>,
        constructor: Box<Self>,
    },
    ClassInit {
        ty: BasicTypeEnum<'ctx>,
        class: BasicTypeEnum<'ctx>,
        constructor_params: Vec<Self>,
    },
}

#[derive(Debug, Clone)]
pub enum TypedNode {
    Statements(Vec<Self>),

    Int {
        ty: Type,
        val: i128,
    },
    Float {
        ty: Type,
        val: f64,
    },
    Boolean {
        ty: Type,
        val: bool,
    },
    Char {
        ty: Type,
        val: char,
    },
    String {
        ty: Type,
        val: String,
    },
    Unary {
        ty: Type,
        val: Box<Self>,
        op_token: Token,
    },
    Binary {
        ty: Type,
        left: Box<Self>,
        right: Box<Self>,
        op_token: Token,
    },
    Fun {
        ty: Type,
        name: String,
        params: Vec<Binder>,
        body: Box<Self>,
    },
    Let {
        ty: Type,
        name: String,
        val: Box<Self>,
    },
    ReLet {
        ty: Type,
        prev: Type,
        name: String,
        val: Box<Self>,
    },
    Var {
        ty: Type,
        name: String,
    },
    Call {
        ty: Type,
        fun: Box<Self>,
        args: Vec<Self>,
    },
    Return {
        ty: Type,
        val: Box<Self>,
    },
    Null {
        ty: Type,
    },
    If {
        ty: Type,
        cases: Vec<(Self, Self)>,
        else_case: Option<Box<Self>>,
    },
    While {
        ty: Type,
        cond: Box<Self>,
        body: Box<Self>,
    },
    For {
        ty: Type,
        var: String,
        start: Box<Self>,
        end: Box<Self>,
        step: Box<Self>,
        body: Box<Self>,
    },
    Array {
        ty: Type,
        elements: Vec<Self>,
    },
    Index {
        ty: Type,
        array: Box<Self>,
        idx: Box<Self>,
    },
    Object {
        ty: Type,
        properties: BTreeMap<String, Self>,
    },
    ObjectAccess {
        ty: Type,
        object: Box<Self>,
        property: String,
    },
    ObjectEdit {
        ty: Type,
        object: Box<Self>,
        new_val: Box<Self>,
        property: String,
    },
    ObjectMethodCall {
        ty: Type,
        object: Box<Self>,
        property: String,
        args: Vec<Self>,
    },
    Class {
        name: String,
        ty: Type,
        properties: BTreeMap<String, Self>,
        methods: BTreeMap<String, Self>,
        constructor: Box<Self>,
    },
    ClassInit {
        ty: Type,
        class: Type,
        constructor_params: Vec<Self>,
    },
}

impl TypedNode {
    pub fn get_type(&self) -> Type {
        match self {
            TypedNode::Statements(stmts) => {
                for stmt in stmts {
                    if let TypedNode::Return { ty, .. } = stmt {
                        return ty.clone();
                    }
                }
                return Type::Null;
            }
            TypedNode::Int { ty, .. }
            | TypedNode::Float { ty, .. }
            | TypedNode::Boolean { ty, .. }
            | TypedNode::Char { ty, .. }
            | TypedNode::String { ty, .. }
            | TypedNode::Fun { ty, .. }
            | TypedNode::Let { ty, .. }
            | TypedNode::ReLet { ty, .. }
            | TypedNode::Var { ty, .. }
            | TypedNode::Call { ty, .. }
            | TypedNode::Return { ty, .. }
            | TypedNode::Unary { ty, .. }
            | TypedNode::Binary { ty, .. }
            | TypedNode::Null { ty }
            | TypedNode::If { ty, .. }
            | TypedNode::While { ty, .. }
            | TypedNode::For { ty, .. }
            | TypedNode::Array { ty, .. }
            | TypedNode::Index { ty, .. }
            | TypedNode::Object { ty, .. }
            | TypedNode::ObjectAccess { ty, .. }
            | TypedNode::ObjectEdit { ty, .. }
            | TypedNode::ObjectMethodCall { ty, .. }
            | TypedNode::Class { ty, .. }
            | TypedNode::ClassInit { ty, .. } => ty.clone(),
        }
    }
}

static mut I: i32 = 0;
static mut OBJECT_ALIGNER: usize = 0;

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub enum Type {
    Int,
    Float,
    Boolean,
    Char,
    String,
    Array(Box<Self>, usize),
    Fun(Vec<Self>, Box<Self>),
    Object(BTreeMap<String, Self>),
    Class(Box<Self>),
    Null,

    Var(i32),
}

impl Type {
    pub fn fresh_var() -> Self {
        let ret = unsafe { Self::Var(I) };
        unsafe { I += 1 };
        ret
    }

    pub fn create_obj(props: BTreeMap<String, Self>) -> Self {
        unsafe {
            OBJECT_ALIGNER += 1;
            let mut tree = BTreeMap::new();
            tree.insert(
                "%alignment%".to_string(),
                Type::Array(Box::new(Type::Int), OBJECT_ALIGNER),
            );
            tree.extend(props);
            Self::Object(tree)
        }
    }

    pub fn last_aligner() -> usize {
        unsafe {
            return OBJECT_ALIGNER;
        }
    }

    pub fn llvm<'ctx>(
        &self,
        ctx: &'ctx Context,
        tvars: BTreeMap<Type, Type>,
    ) -> BasicTypeEnum<'ctx> {
        match self {
            Type::Int => ctx.i128_type().into(),
            Type::Float => ctx.f64_type().into(),
            Type::Boolean => ctx.bool_type().into(),
            Type::Char => ctx.i8_type().into(),
            Type::String => ctx.i8_type().ptr_type(AddressSpace::Generic).into(),
            Type::Array(ty, i) => ty.llvm(ctx, tvars).array_type(*i as u32).into(),
            Type::Fun(params, ret) => ret
                .llvm(ctx, tvars.clone())
                .fn_type(
                    &params
                        .iter()
                        .map(|x| x.llvm(ctx, tvars.clone()))
                        .collect::<Vec<BasicTypeEnum>>()[..],
                    false,
                )
                .ptr_type(AddressSpace::Generic)
                .into(),
            Type::Null => ctx.struct_type(&[], true).into(),
            Type::Var(tvar) => {
                tvars
                    .clone()
                    .get(&Type::Var(*tvar))
                    .unwrap()
                    .llvm(ctx, tvars)
            },
            Type::Object(tree) => ctx
                .struct_type(
                    &tree
                        .iter()
                        .map(|(_, ty)| ty.llvm(ctx, tvars.clone()))
                        .collect::<Vec<BasicTypeEnum>>()[..],
                    false,
                )
                .ptr_type(AddressSpace::Generic)
                .into(),
            Type::Class(obj) => obj.llvm(ctx, tvars),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Binder {
    pub ty: Type,
    pub name: String,
}
