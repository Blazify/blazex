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

use bzs_shared::{ByteCode, Constants, DynType, Node, Tokens};
use std::collections::HashMap;

#[derive(Debug)]
pub enum OpCode {
    OpConstant(u16),
    OpPlus,
    OpMinus,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpPower,
    OpNot,
    OpAnd,
    OpOr,
    OpEquals,
    OpNotEquals,
    OpGreaterThan,
    OpGreaterThanEquals,
    OpLessThan,
    OpLessThanEquals,
    OpVarAssign(u16),
    OpVarAccess(u16),
    OpVarReassign(u16),
    OpJump(u16),
    OpJumpIfFalse(u16),
    OpCall,
    OpBlockStart,
    OpBlockEnd,
    OpIndexArray,
    OpPropertyAccess(u16),
    OpPropertyAssign(u16),
    OpPop,
}

impl OpCode {
    pub fn make_op(&self) -> Vec<u8> {
        match self {
            Self::OpConstant(arg) => make_three_byte_op(0x01, *arg),
            Self::OpPop => vec![0x02],
            Self::OpAdd => vec![0x03],
            Self::OpSubtract => vec![0x04],
            Self::OpMultiply => vec![0x05],
            Self::OpDivide => vec![0x06],
            Self::OpPower => vec![0x07],
            Self::OpJump(to) => make_three_byte_op(0x08, *to),
            Self::OpJumpIfFalse(to) => make_three_byte_op(0x09, *to),
            Self::OpPlus => vec![0x0A],
            Self::OpMinus => vec![0x0B],
            Self::OpNot => vec![0x0C],
            Self::OpAnd => vec![0x0D],
            Self::OpOr => vec![0x0E],
            Self::OpEquals => vec![0x0F],
            Self::OpNotEquals => vec![0x1A],
            Self::OpGreaterThan => vec![0x1B],
            Self::OpGreaterThanEquals => vec![0x1C],
            Self::OpLessThan => vec![0x1D],
            Self::OpLessThanEquals => vec![0x1E],
            Self::OpVarAssign(i) => make_three_byte_op(0x1F, *i),
            Self::OpVarAccess(i) => make_three_byte_op(0x2A, *i),
            Self::OpVarReassign(i) => make_three_byte_op(0x2B, *i),
            Self::OpBlockStart => vec![0x2C],
            Self::OpBlockEnd => vec![0x2D],
            Self::OpCall => vec![0x2E],
            Self::OpIndexArray => vec![0x2F],
            Self::OpPropertyAccess(i) => make_three_byte_op(0x3A, *i),
            Self::OpPropertyAssign(i) => make_three_byte_op(0x3B, *i),
        }
    }
}

fn convert_to_u8(integer: u16) -> [u8; 2] {
    [(integer >> 8) as u8, integer as u8]
}

fn make_three_byte_op(code: u8, data: u16) -> Vec<u8> {
    let mut output = vec![code];
    output.extend(&convert_to_u8(data));
    output
}

#[derive(Debug, Clone)]
pub struct ByteCodeGen {
    pub bytecode: ByteCode,
    pub variables: HashMap<String, u16>,
}

impl ByteCodeGen {
    pub fn new() -> Self {
        Self {
            bytecode: ByteCode::new(),
            variables: HashMap::new(),
        }
    }

    fn variable(&mut self, k: String) -> u16 {
        if self.variables.contains_key(&k) {
            self.variables.get(&k).unwrap().clone()
        } else {
            let idx = self.variables.len().clone();
            self.variables
                .insert(k.clone(), (idx + if idx == 0 { 0 } else { 1 }) as u16);
            self.variables.get(&k).unwrap().clone()
        }
    }

    fn add_constant(&mut self, c: Constants) -> u16 {
        self.bytecode.constants.push(c);
        (self.bytecode.constants.len() - 1) as u16
    }

    fn add_instruction(&mut self, op: OpCode) -> u16 {
        let pos = self.bytecode.instructions.len() as u16;
        self.bytecode.instructions.extend(op.make_op());
        pos
    }

    pub fn compile_node(&mut self, node: Node) {
        match node {
            Node::Statements { statements } => {
                for statement in statements {
                    self.compile_node(statement);
                    self.add_instruction(OpCode::OpPop);
                }
            }
            Node::NumberNode { token } => {
                if token.r#type == Tokens::Int {
                    let idx = self.add_constant(Constants::Int(token.value.into_int()));
                    self.add_instruction(OpCode::OpConstant(idx));
                } else {
                    let idx = self.add_constant(Constants::Float(token.value.into_float()));
                    self.add_instruction(OpCode::OpConstant(idx));
                }
            }
            Node::StringNode { token } => {
                let idx = self.add_constant(Constants::String(token.value.into_string()));
                self.add_instruction(OpCode::OpConstant(idx));
            }
            Node::CharNode { token } => {
                let idx = self.add_constant(Constants::Char(token.value.into_char()));
                self.add_instruction(OpCode::OpConstant(idx));
            }
            Node::BooleanNode { token } => {
                let idx = self.add_constant(Constants::Boolean(token.value.into_boolean()));
                self.add_instruction(OpCode::OpConstant(idx));
            }
            Node::BinOpNode {
                left,
                right,
                op_token,
            } => {
                self.compile_node(*left);
                self.compile_node(*right);

                match op_token.r#type {
                    Tokens::Plus => self.add_instruction(OpCode::OpAdd),
                    Tokens::Minus => self.add_instruction(OpCode::OpSubtract),
                    Tokens::Multiply => self.add_instruction(OpCode::OpMultiply),
                    Tokens::Divide => self.add_instruction(OpCode::OpDivide),
                    Tokens::Power => self.add_instruction(OpCode::OpPower),
                    Tokens::DoubleEquals => self.add_instruction(OpCode::OpEquals),
                    Tokens::NotEquals => self.add_instruction(OpCode::OpNotEquals),
                    Tokens::GreaterThan => self.add_instruction(OpCode::OpGreaterThan),
                    Tokens::GreaterThanEquals => self.add_instruction(OpCode::OpGreaterThanEquals),
                    Tokens::LessThan => self.add_instruction(OpCode::OpLessThan),
                    Tokens::LessThanEquals => self.add_instruction(OpCode::OpLessThanEquals),
                    _ => 0,
                };

                if op_token.matches(Tokens::Keyword, DynType::String("and".to_string())) {
                    self.add_instruction(OpCode::OpAnd);
                } else if op_token.matches(Tokens::Keyword, DynType::String("or".to_string())) {
                    self.add_instruction(OpCode::OpOr);
                }
            }
            Node::UnaryNode { node, op_token } => {
                self.compile_node(*node);

                match op_token.r#type {
                    Tokens::Plus => self.add_instruction(OpCode::OpPlus),
                    Tokens::Minus => self.add_instruction(OpCode::OpMinus),
                    _ => 0,
                };

                if op_token.matches(Tokens::Keyword, DynType::String("not".to_string())) {
                    self.add_instruction(OpCode::OpNot);
                }
            }
            Node::VarAssignNode {
                name,
                value,
                reassignable,
            } => {
                self.compile_node(*value);
                let idx = self.add_constant(Constants::Boolean(reassignable));
                self.add_instruction(OpCode::OpConstant(idx));
                let id = self.variable(name.value.into_string());
                self.add_instruction(OpCode::OpVarAssign(id));
            }
            Node::VarAccessNode { token, .. } => {
                let id = self.variable(token.value.into_string());
                self.add_instruction(OpCode::OpVarAccess(id));
            }
            Node::VarReassignNode { name, value, .. } => {
                self.compile_node(*value);
                let id = self.variable(name.value.into_string());
                self.add_instruction(OpCode::OpVarReassign(id));
            }
            Node::IfNode { cases, else_case } => {
                let mut jumps = vec![];

                for (expr, body) in cases {
                    self.compile_node(expr.clone());
                    let idx = self.add_instruction(OpCode::OpJumpIfFalse(0));
                    self.add_instruction(OpCode::OpBlockStart);
                    self.compile_node(body.clone());
                    self.add_instruction(OpCode::OpBlockEnd);
                    let idx_1 = self.add_instruction(OpCode::OpJump(0));
                    jumps.push(idx_1);
                    self.patch_jump_if_false(idx, None);
                }

                if else_case.is_some() {
                    self.add_instruction(OpCode::OpBlockStart);
                    self.compile_node(else_case.unwrap());
                    self.add_instruction(OpCode::OpBlockEnd);
                }

                for jump in jumps {
                    self.patch_jump(jump, None);
                }
            }
            Node::ForNode {
                var_name_token,
                start_value,
                step_value_node,
                end_value,
                body_node,
            } => {
                self.compile_node(*start_value);
                let idx = self.add_constant(Constants::Boolean(true));
                self.add_instruction(OpCode::OpConstant(idx));
                let id_1 = self.variable(var_name_token.value.into_string());
                self.add_instruction(OpCode::OpVarAssign(id_1));

                let init = self.bytecode.instructions.len();

                let id_2 = self.variable(var_name_token.value.into_string());
                self.add_instruction(OpCode::OpVarAccess(id_2));
                self.compile_node(*end_value);
                self.add_instruction(OpCode::OpNotEquals);

                let idx_3 = self.add_instruction(OpCode::OpJumpIfFalse(1));

                let id_3 = self.variable(var_name_token.value.into_string());
                self.add_instruction(OpCode::OpVarAccess(id_3));
                self.compile_node(*step_value_node);
                self.add_instruction(OpCode::OpAdd);

                let id_4 = self.variable(var_name_token.value.into_string());
                self.add_instruction(OpCode::OpVarReassign(id_4));

                self.add_instruction(OpCode::OpBlockStart);
                self.compile_node(*body_node.clone());
                self.add_instruction(OpCode::OpBlockEnd);
                let jmp = self.add_instruction(OpCode::OpJump(0));
                self.patch_jump_if_false(idx_3, None);
                self.patch_jump(jmp, Some(init as u16));
            }
            Node::WhileNode {
                condition_node,
                body_node,
            } => {
                let init = self.bytecode.instructions.len();
                self.compile_node(*condition_node.clone());
                let idx = self.add_instruction(OpCode::OpJumpIfFalse(0));
                self.add_instruction(OpCode::OpBlockStart);
                self.compile_node(*body_node.clone());
                self.add_instruction(OpCode::OpBlockEnd);
                let jmp = self.add_instruction(OpCode::OpJump(0));
                self.patch_jump_if_false(idx, None);
                self.patch_jump(jmp, Some(init as u16));
            }
            Node::FunDef {
                name,
                body_node,
                arg_tokens,
            } => {
                let mut func_byte = ByteCodeGen::new();
                func_byte.variables = self.variables.clone();
                let mut args: Vec<u16> = vec![];
                for arg in arg_tokens {
                    let id = func_byte.variable(arg.value.into_string());
                    args.push(id);
                }
                func_byte.compile_node(*body_node);
                self.variables = func_byte.variables;
                let idx = self.add_constant(Constants::Function(args, func_byte.bytecode));
                self.add_instruction(OpCode::OpConstant(idx));
                if name.is_some() {
                    let idx_ = self.add_constant(Constants::Boolean(false));
                    self.add_instruction(OpCode::OpConstant(idx_));
                    let id = self.variable(name.unwrap().value.into_string());
                    self.add_instruction(OpCode::OpVarAssign(id));
                }
            }
            Node::CallNode { node_to_call, args } => {
                self.add_instruction(OpCode::OpBlockStart);
                for arg in args {
                    self.compile_node(arg);
                }
                self.compile_node(*node_to_call);
                self.add_instruction(OpCode::OpCall);
                self.add_instruction(OpCode::OpBlockEnd);
            }
            Node::ArrayNode { element_nodes } => {
                let mut array = vec![];
                for element in element_nodes {
                    let mut array_btc = ByteCodeGen::new();
                    array_btc.compile_node(element);
                    array.push(array_btc.bytecode);
                }
                let idx = self.add_constant(Constants::Array(array));
                self.add_instruction(OpCode::OpConstant(idx));
            }
            Node::ArrayAcess { array, index } => {
                self.compile_node(*array);
                self.compile_node(*index);
                self.add_instruction(OpCode::OpIndexArray);
            }
            Node::ObjectDefNode { properties } => {
                let mut compiled_properties = HashMap::new();
                for (k, v) in &properties {
                    let id = self.variable(k.value.into_string());
                    let mut val_btc = ByteCodeGen::new();
                    val_btc.variables = self.variables.clone();
                    val_btc.compile_node(v.clone());
                    self.variables = val_btc.variables.clone();
                    compiled_properties.insert(id as usize, val_btc.bytecode);
                }
                let idx = self.add_constant(Constants::Object(compiled_properties));
                self.add_instruction(OpCode::OpConstant(idx));
            }
            Node::ObjectPropAccess { object, property } => {
                self.compile_node(*object);
                let id = self.variable(property.value.into_string());
                self.add_instruction(OpCode::OpPropertyAccess(id));
            }
            Node::ObjectPropEdit {
                object,
                new_val,
                property,
            } => {
                self.compile_node(*object);
                self.compile_node(*new_val);
                let id = self.variable(property.value.into_string());
                self.add_instruction(OpCode::OpPropertyAssign(id));
            }
            Node::ReturnNode { .. } => {
                todo!("Node::ReturnNode")
            }
            Node::ClassDefNode { .. } => {
                todo!("Node::ClassDefNode")
            }
            Node::ClassInitNode { .. } => {
                todo!("Node::ClassInitNode")
            }
        }
    }

    fn patch_jump_if_false(&mut self, idx: u16, new: Option<u16>) {
        let jump_temp = if new.is_none() {
            let offset = self.bytecode.instructions.len();
            OpCode::OpJumpIfFalse(offset as u16).make_op()
        } else {
            OpCode::OpJumpIfFalse(new.unwrap()).make_op()
        };
        self.bytecode.instructions[(idx + 1) as usize] = jump_temp[1];
        self.bytecode.instructions[(idx + 2) as usize] = jump_temp[2];
    }

    fn patch_jump(&mut self, idx: u16, new: Option<u16>) {
        let jump_temp = if new.is_none() {
            let offset = self.bytecode.instructions.len();
            OpCode::OpJump(offset as u16).make_op()
        } else {
            OpCode::OpJump(new.unwrap()).make_op()
        };
        self.bytecode.instructions[(idx + 1) as usize] = jump_temp[1];
        self.bytecode.instructions[(idx + 2) as usize] = jump_temp[2];
    }
}
