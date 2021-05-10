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
use crate::compiler::bytecode::opcode::OpCode;
use crate::compiler::parser::nodes::Node;
use crate::compiler::token::Token;
use crate::utils::constants::{DynType, Tokens};
use crate::utils::position::Position;
use crate::LanguageServer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Error as E, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Constants {
    Int(i128),
    Float(f64),
    String(String),
    Char(char),
    Boolean(bool),
    Function(Vec<u16>, ByteCode),
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

#[derive(Debug, Clone)]
pub struct ByteCodeGen {
    pub bytecode: ByteCode,
    variables: HashMap<String, u16>,
}

impl LanguageServer for ByteCodeGen {
    type Result = Result<ByteCode, ()>;

    fn from_ast(node: Node) -> Self::Result {
        let mut gen = ByteCodeGen::new();
        gen.compile_node(node);
        Ok(gen.bytecode)
    }
}

impl ByteCodeGen {
    fn new() -> Self {
        Self {
            bytecode: ByteCode::new(),
            variables: HashMap::new(),
        }
    }

    fn variable(&mut self, k: String) -> u16 {
        return *self.variables.clone().get(&k).unwrap_or_else(|| -> &u16 {
            self.variables.insert(
                k.clone(),
                *self.variables.values().last().unwrap_or(&0)
                    + if self.variables.values().len() == 0 {
                        0
                    } else {
                        1
                    },
            );
            self.variables.get(&k).unwrap()
        });
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
            Node::IfNode {
                mut cases,
                else_case,
            } => {
                let mut jumps = vec![];
                if else_case.is_some() {
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
                    let idx = self.add_instruction(OpCode::OpJumpIfFalse(0));
                    self.add_instruction(OpCode::OpBlockStart);
                    self.compile_node(body.clone());
                    self.add_instruction(OpCode::OpBlockEnd);
                    let idx_1 = self.add_instruction(OpCode::OpJump(0));
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
                let id = self.variable(var_name_token.value.into_string());
                self.add_instruction(OpCode::OpVarAssign(id));

                let init = self.bytecode.instructions.len();

                let id = self.variable(var_name_token.value.into_string());
                self.add_instruction(OpCode::OpVarAccess(id));
                self.compile_node(*end_value);
                self.add_instruction(OpCode::OpNotEquals);

                let idx_3 = self.add_instruction(OpCode::OpJumpIfFalse(1));

                let id = self.variable(var_name_token.value.into_string());
                self.add_instruction(OpCode::OpVarAccess(id));
                self.compile_node(*step_value_node);
                self.add_instruction(OpCode::OpAdd);

                let id = self.variable(var_name_token.value.into_string());
                self.add_instruction(OpCode::OpVarReassign(id));

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
                func_byte.compile_node(*body_node);
                let args: Vec<u16> = arg_tokens
                    .iter()
                    .map(|x| self.variable(x.value.into_string()))
                    .collect();
                let idx = self.add_constant(Constants::Function(args, func_byte.bytecode));
                self.add_instruction(OpCode::OpConstant(idx));
                if name.is_some() {
                    let idx = self.add_constant(Constants::Boolean(true));
                    self.add_instruction(OpCode::OpConstant(idx));
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
            _ => panic!("Please don't use 'bytecode' argument for this program."),
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
