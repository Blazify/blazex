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
use crate::compiler::token::Token;

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
