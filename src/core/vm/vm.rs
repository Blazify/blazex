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
                    _ => panic!("Unknown types to OpAdd"),
                },
                0x04 => match (self.pop(), self.pop()) {
                    (Constants::Int(rhs), Constants::Int(lhs)) => {
                        self.push(Constants::Int(lhs - rhs))
                    }
                    _ => panic!("Unknown types to OpSub"),
                },
                0x05 => match (self.pop(), self.pop()) {
                    (Constants::Int(rhs), Constants::Int(lhs)) => {
                        self.push(Constants::Int(lhs * rhs))
                    }
                    _ => panic!("Unknown types to OpMultiply"),
                },
                0x06 => match (self.pop(), self.pop()) {
                    (Constants::Int(rhs), Constants::Int(lhs)) => {
                        self.push(Constants::Int(lhs / rhs))
                    }
                    _ => panic!("Unknown types to OpDivide"),
                },
                0x07 => match (self.pop(), self.pop()) {
                    (Constants::Int(rhs), Constants::Int(lhs)) => {
                        self.push(Constants::Int(lhs.pow(rhs as u32)))
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
