use crate::core::bytecode::{
    bytecode::{ByteCode, Constants},
    opcode::convert_to_usize,
};

const STACK_SIZE: usize = 512;

pub struct VM {
    bytecode: ByteCode,
    stack: [Constants; STACK_SIZE],
    stack_ptr: usize,
}

impl VM {
    pub fn new(bytecode: ByteCode) -> Self {
        Self {
            bytecode,
            stack: unsafe { std::mem::zeroed() },
            stack_ptr: 0,
        }
    }

    pub fn run(&mut self) {
        let mut ip = 0;
        while ip < self.bytecode.instructions.len() {
            let address = ip;
            ip += 1;

            match self.bytecode.instructions[address] {
                0x01 => {
                    let idx = convert_to_usize(
                        self.bytecode.instructions[ip],
                        self.bytecode.instructions[ip + 1],
                    );
                    ip += 2;
                    self.push(self.bytecode.constants[idx].clone());
                }
                0x02 => {
                    self.pop();
                }
                0x03 => match (self.pop(), self.pop()) {
                    (Constants::Int(rhs), Constants::Int(lhs)) => {
                        self.push(Constants::Int(lhs + rhs))
                    }
                    (Constants::Float(rhs), Constants::Float(lhs)) => {
                        self.push(Constants::Float(lhs + rhs))
                    }
                    (Constants::String(rhs), Constants::String(lhs)) => {
                        self.push(Constants::String(lhs + &rhs))
                    }
                    _ => panic!("Unknown types to OpAdd"),
                },
                0x04 => match (self.pop(), self.pop()) {
                    (Constants::Int(rhs), Constants::Int(lhs)) => {
                        self.push(Constants::Int(lhs - rhs))
                    }
                    (Constants::Float(rhs), Constants::Float(lhs)) => {
                        self.push(Constants::Float(lhs - rhs))
                    }
                    _ => panic!("Unknown types to OpSub"),
                },
                0x05 => match (self.pop(), self.pop()) {
                    (Constants::Int(rhs), Constants::Int(lhs)) => {
                        self.push(Constants::Int(lhs * rhs))
                    }
                    (Constants::Float(rhs), Constants::Float(lhs)) => {
                        self.push(Constants::Float(lhs * rhs))
                    }
                    (Constants::Int(rhs), Constants::String(lhs)) => {
                        self.push(Constants::String(lhs.repeat(rhs as usize)))
                    }
                    _ => panic!("Unknown types to OpMultiply"),
                },
                0x06 => match (self.pop(), self.pop()) {
                    (Constants::Int(rhs), Constants::Int(lhs)) => {
                        self.push(Constants::Int(lhs / rhs))
                    }
                    (Constants::Float(rhs), Constants::Float(lhs)) => {
                        self.push(Constants::Float(lhs / rhs))
                    }
                    (Constants::Int(rhs), Constants::String(lhs)) => self.push(Constants::String(
                        (lhs.as_bytes()[rhs as usize] as char).to_string(),
                    )),
                    _ => panic!("Unknown types to OpDivide"),
                },
                0x07 => match (self.pop(), self.pop()) {
                    (Constants::Int(rhs), Constants::Int(lhs)) => {
                        self.push(Constants::Int(lhs.pow(rhs as u32)))
                    }
                    (Constants::Float(rhs), Constants::Float(lhs)) => {
                        self.push(Constants::Float(lhs.powf(rhs)))
                    }
                    _ => panic!("Unknown types to OpPower"),
                },
                0x0A => match self.pop() {
                    Constants::Int(num) => self.push(Constants::Int(num)),
                    _ => panic!("Unknown arg type to OpPlus"),
                },
                0x0B => match self.pop() {
                    Constants::Int(num) => self.push(Constants::Int(-num)),
                    _ => panic!("Unknown arg type to OpMinus"),
                },
                0x0C => match self.pop() {
                    Constants::Boolean(boolean) => self.push(Constants::Boolean(!boolean)),
                    _ => panic!("Unknown arg type to OpNot"),
                },
                0x0D => match (self.pop(), self.pop()) {
                    (Constants::Boolean(rhs), Constants::Boolean(lhs)) => {
                        self.push(Constants::Boolean(lhs && rhs))
                    }
                    _ => panic!("Unknown types to OpAnd"),
                },
                0x0E => match (self.pop(), self.pop()) {
                    (Constants::Boolean(rhs), Constants::Boolean(lhs)) => {
                        self.push(Constants::Boolean(lhs || rhs))
                    }
                    _ => panic!("Unknown types to OpAnd"),
                },
                0xAB => match (self.pop(), self.pop()) {
                    (Constants::Int(rhs), Constants::Int(lhs)) => {
                        self.push(Constants::Boolean(lhs == rhs))
                    }
                    (Constants::Float(rhs), Constants::Float(lhs)) => {
                        self.push(Constants::Boolean(lhs == rhs))
                    }
                    (Constants::String(rhs), Constants::String(lhs)) => {
                        self.push(Constants::Boolean(lhs == rhs))
                    }
                    (Constants::Char(rhs), Constants::Char(lhs)) => {
                        self.push(Constants::Boolean(lhs == rhs))
                    }
                    (Constants::Boolean(rhs), Constants::Boolean(lhs)) => {
                        self.push(Constants::Boolean(lhs == rhs))
                    }
                    _ => panic!("Unknown types to OpEquals"),
                },
                0xAC => match (self.pop(), self.pop()) {
                    (Constants::Int(rhs), Constants::Int(lhs)) => {
                        self.push(Constants::Boolean(lhs != rhs))
                    }
                    (Constants::Float(rhs), Constants::Float(lhs)) => {
                        self.push(Constants::Boolean(lhs != rhs))
                    }
                    (Constants::String(rhs), Constants::String(lhs)) => {
                        self.push(Constants::Boolean(lhs != rhs))
                    }
                    (Constants::Char(rhs), Constants::Char(lhs)) => {
                        self.push(Constants::Boolean(lhs != rhs))
                    }
                    (Constants::Boolean(rhs), Constants::Boolean(lhs)) => {
                        self.push(Constants::Boolean(lhs != rhs))
                    }
                    _ => panic!("Unknown types to OpNotEquals"),
                },
                0xAD => match (self.pop(), self.pop()) {
                    (Constants::Int(rhs), Constants::Int(lhs)) => {
                        self.push(Constants::Boolean(lhs < rhs))
                    }
                    (Constants::Float(rhs), Constants::Float(lhs)) => {
                        self.push(Constants::Boolean(lhs < rhs))
                    }
                    (Constants::String(rhs), Constants::String(lhs)) => {
                        self.push(Constants::Boolean(lhs < rhs))
                    }
                    (Constants::Char(rhs), Constants::Char(lhs)) => {
                        self.push(Constants::Boolean(lhs < rhs))
                    }
                    (Constants::Boolean(rhs), Constants::Boolean(lhs)) => {
                        self.push(Constants::Boolean(lhs < rhs))
                    }
                    _ => panic!("Unknown types to OpGreaterThan"),
                },
                0xAE => match (self.pop(), self.pop()) {
                    (Constants::Int(rhs), Constants::Int(lhs)) => {
                        self.push(Constants::Boolean(lhs <= rhs))
                    }
                    (Constants::Float(rhs), Constants::Float(lhs)) => {
                        self.push(Constants::Boolean(lhs <= rhs))
                    }
                    (Constants::String(rhs), Constants::String(lhs)) => {
                        self.push(Constants::Boolean(lhs <= rhs))
                    }
                    (Constants::Char(rhs), Constants::Char(lhs)) => {
                        self.push(Constants::Boolean(lhs <= rhs))
                    }
                    (Constants::Boolean(rhs), Constants::Boolean(lhs)) => {
                        self.push(Constants::Boolean(lhs <= rhs))
                    }
                    _ => panic!("Unknown types to OpGreaterThanEquals"),
                },
                0xBA => match (self.pop(), self.pop()) {
                    (Constants::Int(rhs), Constants::Int(lhs)) => {
                        self.push(Constants::Boolean(lhs > rhs))
                    }
                    (Constants::Float(rhs), Constants::Float(lhs)) => {
                        self.push(Constants::Boolean(lhs > rhs))
                    }
                    (Constants::String(rhs), Constants::String(lhs)) => {
                        self.push(Constants::Boolean(lhs > rhs))
                    }
                    (Constants::Char(rhs), Constants::Char(lhs)) => {
                        self.push(Constants::Boolean(lhs > rhs))
                    }
                    (Constants::Boolean(rhs), Constants::Boolean(lhs)) => {
                        self.push(Constants::Boolean(lhs > rhs))
                    }
                    _ => panic!("Unknown types to OpNotEquals"),
                },
                0xBB => match (self.pop(), self.pop()) {
                    (Constants::Int(rhs), Constants::Int(lhs)) => {
                        self.push(Constants::Boolean(lhs >= rhs))
                    }
                    (Constants::Float(rhs), Constants::Float(lhs)) => {
                        self.push(Constants::Boolean(lhs >= rhs))
                    }
                    (Constants::String(rhs), Constants::String(lhs)) => {
                        self.push(Constants::Boolean(lhs >= rhs))
                    }
                    (Constants::Char(rhs), Constants::Char(lhs)) => {
                        self.push(Constants::Boolean(lhs >= rhs))
                    }
                    (Constants::Boolean(rhs), Constants::Boolean(lhs)) => {
                        self.push(Constants::Boolean(lhs >= rhs))
                    }
                    _ => panic!("Unknown types to OpNotEquals"),
                },
                _ => panic!("Unknown instruction"),
            }
        }
    }

    pub fn push(&mut self, node: Constants) {
        self.stack[self.stack_ptr] = node;
        self.stack_ptr += 1;
    }

    pub fn pop(&mut self) -> Constants {
        let node = self.stack[self.stack_ptr - 1].clone();
        self.stack_ptr -= 1;
        node
    }
    pub fn pop_last(&self) -> &Constants {
        &self.stack[self.stack_ptr]
    }
}
