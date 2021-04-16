use crate::core::compiler::op_code::{make_op, OpCode};
use crate::utils::constants::Tokens;
use crate::Compile;
use crate::Node;

#[derive(Debug)]
struct Bytecode {
    pub instructions: Vec<u8>,
    pub constants: Vec<String>,
}

impl Bytecode {
    fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct BytecodeGen {
    bytecode: Bytecode,
}

impl Compile for BytecodeGen {
    fn from_ast(node: &Node) -> Result<String, String> {
        let mut interpreter = BytecodeGen {
            bytecode: Bytecode::new(),
        };
        interpreter.interpret_node(node.clone());
        interpreter.add_instruction(OpCode::OpPop);
        Ok(format!(
            "Instructions: {:?}, Bytecode: {:?}",
            interpreter.bytecode.instructions, interpreter.bytecode.constants
        ))
    }
}

impl BytecodeGen {
    fn add_constant(&mut self, node: String) -> u16 {
        self.bytecode.constants.push(node);
        (self.bytecode.constants.len() - 1) as u16
    }

    fn add_instruction(&mut self, op_code: OpCode) -> u16 {
        let position_of_new_instruction = self.bytecode.instructions.len() as u16;
        self.bytecode.instructions.extend(make_op(op_code));
        position_of_new_instruction
    }

    fn interpret_node(&mut self, node: Node) {
        match node {
            Node::NumberNode { token, .. } => {
                let const_index = self.add_constant(format!("{:?}", token.value));
                self.add_instruction(OpCode::OpConstant(const_index));
            }
            Node::UnaryNode { op_token, node, .. } => {
                self.interpret_node(*node);
                match op_token.r#type {
                    Tokens::Plus => self.add_instruction(OpCode::OpPlus),
                    Tokens::Minus => self.add_instruction(OpCode::OpMinus),
                    _ => panic!(),
                };
            }
            Node::BinOpNode {
                op_token,
                left,
                right,
                ..
            } => {
                self.interpret_node(*left);
                self.interpret_node(*right);
                match op_token.r#type {
                    Tokens::Plus => self.add_instruction(OpCode::OpAdd),
                    Tokens::Minus => self.add_instruction(OpCode::OpSub),
                    _ => panic!(),
                };
            }
            _ => panic!(),
        };
    }
}
