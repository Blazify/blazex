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
use crate::compiler::bytecode::{
    bytecode::{ByteCode, Constants},
    opcode::convert_to_usize,
};
use std::collections::HashMap;

const STACK_SIZE: usize = 512;

#[derive(Debug, Clone)]
pub struct VM {
    bytecode: ByteCode,
    stack: [Constants; STACK_SIZE],
    stack_ptr: usize,
    symbols: Vec<HashMap<usize, (Constants, bool)>>,
}

impl VM {
    pub fn new(
        bytecode: ByteCode,
        symbols: Option<Vec<HashMap<usize, (Constants, bool)>>>,
    ) -> Self {
        Self {
            bytecode,
            stack: unsafe { std::mem::zeroed() },
            stack_ptr: 0,
            symbols: if symbols.is_none() {
                vec![HashMap::new()]
            } else {
                symbols.unwrap()
            },
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
                0x08 => {
                    ip = convert_to_usize(
                        self.bytecode.instructions[ip],
                        self.bytecode.instructions[ip + 1],
                    );
                }
                0x09 => match self.pop() {
                    Constants::Boolean(b) => {
                        if !b {
                            ip = convert_to_usize(
                                self.bytecode.instructions[ip],
                                self.bytecode.instructions[ip + 1],
                            );
                        } else {
                            ip += 2;
                        }
                    }
                    _ => panic!("Unknown types to OpJump"),
                },
                0x0A => match self.pop() {
                    Constants::Int(num) => self.push(Constants::Int(num * 1)),
                    Constants::Float(num) => self.push(Constants::Float(num * 1.0)),
                    _ => panic!("Unknown arg type to OpPlus"),
                },
                0x0B => match self.pop() {
                    Constants::Int(num) => self.push(Constants::Int(num * -1)),
                    Constants::Float(num) => self.push(Constants::Float(num * -1.0)),
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
                    _ => panic!("Unknown types to OpOr"),
                },
                0x0F => match (self.pop(), self.pop()) {
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
                0x1A => match (self.pop(), self.pop()) {
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
                0x1B => match (self.pop(), self.pop()) {
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
                    _ => panic!("Unknown types to OpGreaterThan"),
                },
                0x1C => match (self.pop(), self.pop()) {
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
                    _ => panic!("Unknown types to OpGreaterThanEquals"),
                },
                0x1D => match (self.pop(), self.pop()) {
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
                    _ => panic!("Unknown types to OpLessThan"),
                },
                0x1E => match (self.pop(), self.pop()) {
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
                    _ => panic!("Unknown types to OpLessThanEquals"),
                },
                0x1F => match self.pop() {
                    Constants::Boolean(b) => {
                        let i = convert_to_usize(
                            self.bytecode.instructions[ip],
                            self.bytecode.instructions[ip + 1],
                        );
                        ip += 2;
                        let n = self.pop();
                        self.symbols.last_mut().unwrap().insert(i, (n, b));
                    }
                    _ => panic!("Unknown types to OpVarAssign"),
                },
                0x2A => {
                    let i = convert_to_usize(
                        self.bytecode.instructions[ip],
                        self.bytecode.instructions[ip + 1],
                    );
                    ip += 2;
                    self.push(self.get_from_hash_table(i).unwrap().0);
                }
                0x2B => {
                    let i = convert_to_usize(
                        self.bytecode.instructions[ip],
                        self.bytecode.instructions[ip + 1],
                    );
                    ip += 2;
                    if self.get_from_hash_table(i).is_none() {
                        panic!("No variable found to be reassigned")
                    }

                    if self.get_from_hash_table(i).unwrap().1 == false {
                        panic!("Variable not reassignable")
                    }

                    let n = self.pop();
                    self.get_and_set_hash_table(i, (n, true));
                }
                0x2C => self.symbols.push(HashMap::new()),
                0x2D => {
                    self.symbols.pop();
                }
                0x2E => match self.pop() {
                    Constants::Function(args, body) => {
                        for arg in args {
                            let eval_arg = self.pop();
                            self.symbols
                                .last_mut()
                                .unwrap()
                                .insert(arg as usize, (eval_arg, true));
                        }
                        let mut fun_vm = VM::new(body, Some(self.symbols.clone()));
                        fun_vm.run();
                        self.push(fun_vm.pop_last().clone());
                        self.symbols = fun_vm.symbols;
                    }
                    _ => panic!("Unknown Types applied to OpCall"),
                },
                _ => panic!(
                    "\nPrevious instruction {}\nCurrent Instruction: {}\nNext Instruction: {}\n",
                    self.bytecode.instructions[address - 1],
                    self.bytecode.instructions[address],
                    self.bytecode.instructions[address + 1]
                ),
            }
        }
    }

    pub fn push(&mut self, node: Constants) {
        self.stack[self.stack_ptr] = node;
        self.stack_ptr += 1;
    }

    pub fn pop(&mut self) -> Constants {
        let node = self.stack[if self.stack_ptr == 0 {
            self.stack_ptr
        } else {
            self.stack_ptr - 1
        }]
        .clone();
        self.stack_ptr -= if self.stack_ptr == 0 { 0 } else { 1 };
        node
    }

    pub fn pop_last(&self) -> &Constants {
        &self.stack[self.stack_ptr]
    }

    pub fn get_from_hash_table(&self, k: usize) -> Option<(Constants, bool)> {
        for idx in (0..self.symbols.len()).rev() {
            let sym = self.symbols.get(idx).unwrap().get(&k);
            if sym.is_some() {
                return Some((sym.unwrap()).clone());
            }
        }
        None
    }

    pub fn get_and_set_hash_table(&mut self, k: usize, n: (Constants, bool)) {
        for idx in (0..self.symbols.len()).rev() {
            let sym = self.symbols.get(idx).unwrap().get(&k.clone());
            if sym.is_some() {
                self.symbols.get_mut(idx).unwrap().insert(k, n);
                break;
            }
        }
    }
}
