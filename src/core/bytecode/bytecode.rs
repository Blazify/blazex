use crate::core::bytecode::opcode::OpCode;
use crate::core::parser::nodes::Node;
use crate::utils::constants::{DynType, Tokens};
use crate::utils::error::Error;
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
            Node::Statements { statements, .. } => {
                for statement in statements {
                    self.compile_node(statement);
                    self.add_instruction(OpCode::OpPop);
                }
            }
            Node::NumberNode { token, .. } => {
                if token.r#type == Tokens::Int {
                    let idx = self.add_constant(Constants::Int(token.value.into_int()));
                    self.add_instruction(OpCode::OpConstant(idx));
                } else {
                    let idx = self.add_constant(Constants::Float(token.value.into_float()));
                    self.add_instruction(OpCode::OpConstant(idx));
                }
            }
            Node::StringNode { token, .. } => {
                let idx = self.add_constant(Constants::String(token.value.into_string()));
                self.add_instruction(OpCode::OpConstant(idx));
            }
            Node::CharNode { token, .. } => {
                let idx = self.add_constant(Constants::Char(token.value.into_char()));
                self.add_instruction(OpCode::OpConstant(idx));
            }
            Node::BooleanNode { token, .. } => {
                let idx = self.add_constant(Constants::Boolean(token.value.into_boolean()));
                self.add_instruction(OpCode::OpConstant(idx));
            }
            Node::BinOpNode {
                left,
                right,
                op_token,
                ..
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
            Node::UnaryNode { node, op_token, .. } => {
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
                ..
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
            _ => (),
        }
    }
}
