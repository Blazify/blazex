/*
   Copyright 2021 Blazify

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
use crate::core::bytecode::opcode::OpCode;
use crate::core::parser::nodes::Node;
use crate::core::token::Token;
use crate::utils::constants::{DynType, Tokens};
use crate::utils::error::Error;
use crate::utils::position::Position;
use crate::LanguageServer;

#[derive(Debug, Clone)]
pub enum Constants {
    Int(i64),
    Float(f32),
    String(String),
    Char(char),
    Boolean(bool),
    Identifier(String),
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct ByteCodeGen {
    pub bytecode: ByteCode,
}

impl LanguageServer for ByteCodeGen {
    type Result = Result<ByteCode, Error>;

    fn from_ast(_name: &'static str, node: Node) -> Self::Result {
        let mut gen = ByteCodeGen::new();
        gen.compile_node(node);
        Ok(gen.bytecode)
    }
}

impl ByteCodeGen {
    fn new() -> Self {
        Self {
            bytecode: ByteCode::new(),
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

    fn compile_node(&mut self, node: Node) {
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
                    _ => 0 as u16,
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
                    _ => 0 as u16,
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
                let idx_2 = self.add_constant(Constants::Identifier(name.value.into_string()));
                self.add_instruction(OpCode::OpConstant(idx_2));
                self.add_instruction(OpCode::OpVarAssign);
            }
            Node::VarAccessNode { token, .. } => {
                let idx = self.add_constant(Constants::Identifier(token.value.into_string()));
                self.add_instruction(OpCode::OpConstant(idx));
                self.add_instruction(OpCode::OpVarAccess);
            }
            Node::VarReassignNode { name, value, .. } => {
                self.compile_node(*value);
                let idx_2 = self.add_constant(Constants::Identifier(name.value.into_string()));
                self.add_instruction(OpCode::OpConstant(idx_2));
                self.add_instruction(OpCode::OpVarReassign);
            }
            Node::IfNode {
                mut cases,
                else_case,
            } => {
                let mut jumps = vec![];
                if else_case.is_some() {
                    // TODO: Remove this when Node enum TODO is complete
                    let pos = Position::new(-1, -1, -1, "", "");
                    cases.push((
                        Node::BooleanNode {
                            token: Token::new(Tokens::Boolean, pos, pos, DynType::Boolean(true)),
                        },
                        else_case.unwrap(),
                    ));
                }
                for (expr, body) in cases {
                    self.compile_node(expr.clone());
                    let idx = self.add_instruction(OpCode::OpJumpIfFalse(1));
                    self.compile_node(body.clone());
                    let idx_1 = self.add_instruction(OpCode::OpJump(1));
                    jumps.push(idx_1);
                    self.patch_jump_if_false(idx, None);
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
                let idx_1 =
                    self.add_constant(Constants::Identifier(var_name_token.value.into_string()));
                self.add_instruction(OpCode::OpConstant(idx_1));
                self.add_instruction(OpCode::OpVarAssign);

                let init = self.bytecode.instructions.len();

                let idx_2 =
                    self.add_constant(Constants::Identifier(var_name_token.value.into_string()));
                self.add_instruction(OpCode::OpConstant(idx_2));
                self.add_instruction(OpCode::OpVarAccess);
                self.compile_node(*end_value);
                self.add_instruction(OpCode::OpLessThanEquals);

                let idx_3 = self.add_instruction(OpCode::OpJumpIfFalse(1));

                let idx_4 =
                    self.add_constant(Constants::Identifier(var_name_token.value.into_string()));
                self.add_instruction(OpCode::OpConstant(idx_4));
                self.add_instruction(OpCode::OpVarAccess);
                if step_value_node.is_some() {
                    self.compile_node(step_value_node.unwrap());
                } else {
                    let int = self.add_constant(Constants::Int(1));
                    self.add_instruction(OpCode::OpConstant(int));
                }
                self.add_instruction(OpCode::OpAdd);

                let idx_5 =
                    self.add_constant(Constants::Identifier(var_name_token.value.into_string()));
                self.add_instruction(OpCode::OpConstant(idx_5));
                self.add_instruction(OpCode::OpVarReassign);

                self.compile_node(*body_node.clone());
                let jmp = self.add_instruction(OpCode::OpJump(1));
                self.patch_jump_if_false(idx_3, None);
                self.patch_jump(jmp, Some(init as u16));
            }
            Node::WhileNode {
                condition_node,
                body_node,
            } => {
                let init = self.bytecode.instructions.len();
                self.compile_node(*condition_node.clone());
                let idx = self.add_instruction(OpCode::OpJumpIfFalse(1));
                self.compile_node(*body_node.clone());
                let jmp = self.add_instruction(OpCode::OpJump(1));
                self.patch_jump_if_false(idx, None);
                self.patch_jump(jmp, Some(init as u16));
            }
            _ => panic!("Please don't use 'bytecode' argument for this program."),
        }
    }

    fn patch_jump_if_false(&mut self, idx: u16, new: Option<u16>) {
        let offset = self.bytecode.instructions.len();
        let jump_temp = if new.is_none() {
            OpCode::OpJumpIfFalse(offset as u16).make_op()
        } else {
            OpCode::OpJumpIfFalse(new.unwrap()).make_op()
        };
        self.bytecode.instructions[(idx + 1) as usize] = jump_temp[1];
        self.bytecode.instructions[(idx + 2) as usize] = jump_temp[2];
    }

    fn patch_jump(&mut self, idx: u16, new: Option<u16>) {
        let offset = self.bytecode.instructions.len();
        let jump_temp = if new.is_none() {
            OpCode::OpJump(offset as u16).make_op()
        } else {
            OpCode::OpJump(new.unwrap()).make_op()
        };
        self.bytecode.instructions[(idx + 1) as usize] = jump_temp[1];
        self.bytecode.instructions[(idx + 2) as usize] = jump_temp[2];
    }
}
